use std::{
    cell::RefCell,
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::krate::toent::logic::{EventBuilder, timeevent::TimeEvent, todoevent::TodoEvent};
use chrono::NaiveDateTime;
use comrak::{
    arena_tree::Node,
    nodes::{Ast, LineColumn, NodeValue},
};
use itertools::Itertools;
use lazy_regex::Lazy;
use regex::Regex;

#[derive(Debug, Clone)]
enum ChnotBlockEnum {
    Heading,
    ListItem,
}

#[derive(Debug, Clone)]
enum PropsEnum {
    ID(String),
    ToentEvent(TimeEvent),
    ToentState { state: String, time: NaiveDateTime },
}

enum InsertType {
    PropsType(PropsEnum),
}

#[derive(Clone, Debug, Copy)]
struct Point {
    column: usize,
    line: usize,
    offset: usize,
}

#[derive(Debug, Clone)]
struct WithPos<E> {
    start_in: Point,
    end_ex: Point,
    original: String,
    data: E,
}

impl<E> Deref for WithPos<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<E> DerefMut for WithPos<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[derive(Debug, Clone)]
struct ChnotBlock {
    title: String,
    block_type: ChnotBlockEnum,
    todo_event: Option<WithPos<TodoEvent>>,
    time_event: Vec<WithPos<TimeEvent>>,
    backlinks: Vec<WithPos<String>>,
    hashtags: Vec<WithPos<String>>,
    props: Vec<WithPos<Props>>,
    end_ex: usize,
    start: Point,
}

#[derive(Clone, Debug)]
enum SpanType {
    TimeEvent(Box<TimeEvent>),
    TodoEvent(TodoEvent),
    Backlink(String),
    ID(String),
    Hashtag(String),
}

#[derive(Clone, Debug)]
struct Span {
    start_ex: Point,
    end_ex: Point,
    span: SpanType,
}

pub struct ChnotParser<'a> {
    original: &'a str,
    chnot_map: HashMap<usize, ChnotBlock>,
}

#[derive(Clone, Debug)]
struct TextLocater<'a> {
    original: &'a str,
    line_ends: Vec<usize>,
}

impl<'a> TextLocater<'a> {
    fn new(text: &'a str) -> Self {
        let mut start = 0;
        let mut ends = vec![];

        while start < text.len() {
            let Some(count) = text[start..].find('\n') else {
                ends.push(text.len());
                break;
            };
            ends.push(start + count);
            start = start + count + 1;
        }

        Self {
            original: text,
            line_ends: ends,
        }
    }

    fn locate_by_linecol(&self, point: LineColumn) -> Point {
        assert!(point.line > 0, "point line should 1-based");
        assert!(point.column > 0, "point column should 1-based");

        let row = point.line;
        let offset = if row == 1 {
            point.column - 1
        } else {
            self.line_ends[row - 2] + 1 // include the `\n` char
            + point.column
                - 1
        };

        Point {
            line: point.line,
            column: point.column,
            offset,
        }
    }

    fn locate_by_offset(&self, offset: usize) -> Option<Point> {
        if offset > self.original.len() {
            return None;
        }

        for (row, end_ex) in self.line_ends.iter().enumerate() {
            let start_in = if row == 0 {
                0
            } else {
                self.line_ends[row - 1] + 1
            };
            if offset > *end_ex || offset < start_in {
                continue;
            }
            let col = offset - start_in;
            return Some(Point {
                line: row,
                column: col,
                offset,
            });
        }
        None
    }
}

static HEADING_TOENT_TODO_REGEX: Lazy<Regex> =
    lazy_regex::lazy_regex!(r"^#+ +\[([A-Z]+( ![A-Z])?)\]");
static LISTITEM_TOENT_TODO_REGEX: Lazy<Regex> =
    lazy_regex::lazy_regex!(r"\s*- +\[([A-Z]+( ![A-Z])?)\]");

static PROPS_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r";+ +([^ :]+): (.*)");

static HASHTAG_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r"#([^\s#\[\]]+)");
static BACKLINK_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r"\[\[([0-9a-zA-Z]+)]]");

#[derive(Debug, Clone)]
struct Props {
    key: String,
    value: String,
}

impl<'a> ChnotParser<'a> {
    pub fn new(text: &'a str) -> Self {
        // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
        let arena = comrak::Arena::new();

        // Parse the document into a root `AstNode`
        let root = comrak::parse_document(&arena, text, &comrak::Options::default());
        let mut res = Self {
            original: text,
            chnot_map: Default::default(),
        };

        res.parse(root);

        res
    }

    fn parse<'b>(&mut self, root: &'b Node<'b, RefCell<Ast>>) {
        #[derive(Default)]
        struct ChnotBlockGather {
            time_event: Vec<WithPos<TimeEvent>>,
            backlinks: Vec<WithPos<String>>,
            hashtags: Vec<WithPos<String>>,
            props: Vec<WithPos<Props>>,
        }

        /// extract toent only from the listitem first line and headline
        fn extract_spans(
            text: &str,
            start_point: &Point,
            spans: &mut ChnotBlockGather,
            locater: &TextLocater<'_>,
        ) {
            for cps in HASHTAG_REGEX.captures_iter(text) {
                if let (Some(cp), Some(inner)) = (cps.get(0), cps.get(1)) {
                    let result = inner.as_str();
                    let start = start_point.offset + cp.start();
                    let end = start_point.offset + cp.end();
                    spans.hashtags.push(WithPos {
                        start_in: locater.locate_by_offset(start).unwrap(),
                        end_ex: locater.locate_by_offset(end).unwrap(),
                        data: result.to_owned(),
                        original: cp.as_str().to_string(),
                    });
                }
            }

            for cps in BACKLINK_REGEX.captures_iter(text) {
                if let (Some(cp), Some(inner)) = (cps.get(0), cps.get(1)) {
                    let result = inner.as_str();
                    let start = start_point.offset + cp.start();
                    let end = start_point.offset + cp.end();
                    spans.backlinks.push(WithPos {
                        start_in: locater.locate_by_offset(start).unwrap(),
                        end_ex: locater.locate_by_offset(end).unwrap(),
                        data: result.to_owned(),
                        original: cp.as_str().to_string(),
                    });
                }
            }

            for (row, line) in text.split("\n").enumerate() {
                let row = start_point.line + row;
                if let Some(prop) = PROPS_REGEX.captures(line)
                    && let (Some(key), Some(value)) = (prop.get(1), prop.get(2))
                {
                    let line_end = start_point.column + line.len();
                    spans.props.push(WithPos {
                        start_in: locater.locate_by_linecol(LineColumn {
                            column: start_point.column,
                            line: row,
                        }),
                        end_ex: locater.locate_by_linecol(LineColumn {
                            column: line_end,
                            line: row,
                        }),
                        data: Props {
                            key: key.as_str().to_string(),
                            value: value.as_str().to_string(),
                        },
                        original: line.to_string(),
                    });
                }
            }
        }

        let locater: TextLocater<'_> = TextLocater::new(self.original);

        let mut starts: Vec<(ChnotBlockEnum, Point)> = vec![];
        let mut spans: ChnotBlockGather = ChnotBlockGather::default();

        for node in root.descendants() {
            let data = node.data.borrow();
            let pos = data.sourcepos;
            match &data.value {
                NodeValue::Item(_) => {
                    starts.push((
                        ChnotBlockEnum::ListItem,
                        locater.locate_by_linecol(pos.start),
                    ));
                }
                NodeValue::Heading(_) => {
                    starts.push((
                        ChnotBlockEnum::Heading,
                        locater.locate_by_linecol(pos.start),
                    ));
                }
                NodeValue::Text(text) => {
                    extract_spans(
                        text,
                        &locater.locate_by_linecol(pos.start),
                        &mut spans,
                        &locater,
                    );
                }
                _ => {}
            }
        }

        starts.sort_by(|(_, s1), (_, s2)| s1.offset.cmp(&s2.offset));

        let lines: Vec<&str> = self.original.split('\n').collect();

        for (id, (bt, start)) in starts.iter().enumerate() {
            let end_ex = if id == starts.len() - 1 {
                self.original.len()
            } else {
                starts[id + 1].1.offset - 1
            };

            let title = lines[start.line - 1];
            let todo_event = if let Some(todo) = LISTITEM_TOENT_TODO_REGEX.captures(title) {
                let todo = todo.get(1);
                match todo {
                    Some(matched) => match TodoEvent::try_from_standrd_str(matched.as_str()) {
                        Ok(te) => {
                            println!("start, {:?} --{} -- {}", start, matched.as_str(), title);
                            Some(WithPos {
                                start_in: locater.locate_by_linecol(LineColumn {
                                    line: start.line,
                                    column: matched.start() + 1,
                                }),
                                end_ex: locater.locate_by_linecol(LineColumn {
                                    line: start.line,
                                    column: matched.end() + 1,
                                }),
                                original: matched.as_str().to_string(),
                                data: te,
                            })
                        }
                        Err(_) => None,
                    },
                    None => None,
                }
            } else if let Some(todo) = HEADING_TOENT_TODO_REGEX.captures(title) {
                let todo = todo.get(1);
                match todo {
                    Some(matched) => match TodoEvent::try_from_standrd_str(matched.as_str()) {
                        Ok(te) => Some(WithPos {
                            start_in: locater.locate_by_linecol(LineColumn {
                                line: start.line,
                                column: matched.start() + 1,
                            }),
                            end_ex: locater.locate_by_linecol(LineColumn {
                                line: start.line,
                                column: matched.end() + 1,
                            }),
                            original: matched.as_str().to_string(),
                            data: te,
                        }),
                        Err(_) => None,
                    },
                    None => None,
                }
            } else {
                None
            };

            let props = spans
                .props
                .iter()
                .filter(|s| s.start_in.offset >= start.offset && s.end_ex.offset < end_ex)
                .cloned()
                .collect();
            let timeevents = spans
                .time_event
                .iter()
                .filter(|s| s.start_in.offset >= start.offset && s.end_ex.offset < end_ex)
                .cloned()
                .collect();
            let backlinks = spans
                .backlinks
                .iter()
                .filter(|s| s.start_in.offset >= start.offset && s.end_ex.offset < end_ex)
                .cloned()
                .collect();
            let hashtags = spans
                .hashtags
                .iter()
                .filter(|s| s.start_in.offset >= start.offset && s.end_ex.offset < end_ex)
                .cloned()
                .collect();

            let chnot_block = ChnotBlock {
                title: title.to_owned(),
                todo_event,
                block_type: bt.to_owned(),
                time_event: timeevents,
                backlinks,
                hashtags,
                end_ex,
                start: *start,
                props,
            };
            self.chnot_map.insert(chnot_block.start.offset, chnot_block);
        }
    }

    pub fn get_all_tags(&self) -> Vec<&str> {
        self.chnot_map
            .iter()
            .flat_map(|(_, c)| c.hashtags.iter())
            .map(|c| c.as_str())
            .unique()
            .collect()
    }

    pub fn get_outer_todo_event(&self) -> Option<TodoEvent> {
        let te: Vec<TodoEvent> = self
            .chnot_map
            .values()
            .filter_map(|c| c.todo_event.as_ref().map(|te| te.data))
            .collect();

        if te.is_empty() {
            return None;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use comrak::nodes::LineColumn;

    use crate::krate::chnot::parser::{ChnotParser, TextLocater};

    #[test]
    fn text_locater_test() {
        let text: &'static str = "123
456
789
0";
        let locater = TextLocater::new(text);
        println!("{:?}", locater.line_ends);
        let point5 = locater.locate_by_linecol(LineColumn { line: 2, column: 2 });
        assert_eq!("5", &text[point5.offset..point5.offset + 1]);
        let point6 = locater.locate_by_offset(6).unwrap();
        assert_eq!("6", &text[point6.offset..point6.offset + 1]);
    }

    const TEST_MARKDOWN: &str = "
# ID、Hashtag 和 Toent 测试
; ID: head-1

Example Text

## [TODO] Head [[backlinkinhead]] #taginhead
; ID: head-1.1
; EVENT: 2025-12-02 12:00:00 ,12d **12d =2025-12-30
; STATE: TODO @ 2025-05-05 12:00:00 +8:00
;; NOTE: Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
; STATE: DOING @ 2025-05-05 12:00:00 +8:00
; STATE: DONE @ 2025-05-05 15:00:00 +8:00

- [DONE !A] List Item 1 [[backlinkinlist]] #taginlist
  ; ID: head-1.1-list-1
  ; STATE: DONE 2025-06-26 11:20:27
  #tag2 [[418192012]] *italic* *斜体* **bold** **加粗** common text after bold
  what did you say? #tag3

  Paragraph 2 {DONE} 检查时间 {2025-06-26} [[34567890]]
      ID: sdgdfgsdg1
  // CLOSED: {2025-06-26 11:20:27}
  ```txt
  [TODO] [[backlink-in-code]] #hashtagincode
  ```

  - [TODO !B] List Item 2 [[backlinkinlist2]] #taginlist2
## last head
    ";

    fn all_included_and_same_size<E: Eq>(a: &[E], b: &[E]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        for ele in a {
            if !b.contains(ele) {
                return false;
            }
        }

        true
    }

    #[test]
    fn mdwt_it() {
        let cp = ChnotParser::new(TEST_MARKDOWN);
        for cb in cp.chnot_map.values() {
            for ele in &cb.backlinks {
                let src = &TEST_MARKDOWN[ele.start_in.offset..ele.end_ex.offset];
                assert_eq!(src, ele.original);
            }

            for ele in &cb.hashtags {
                let src = &TEST_MARKDOWN[ele.start_in.offset..ele.end_ex.offset];
                assert_eq!(src, ele.original);
            }

            for ele in &cb.time_event {
                let src: &str = &TEST_MARKDOWN[ele.start_in.offset..ele.end_ex.offset];

                assert_eq!(src, ele.original);
            }

            if let Some(ele) = &cb.todo_event {
                let src = &TEST_MARKDOWN[ele.start_in.offset..ele.end_ex.offset];
                assert_eq!(src, ele.original);
            }

            for ele in &cb.props {
                let src = &TEST_MARKDOWN[ele.start_in.offset..ele.end_ex.offset];
                assert_eq!(src, ele.original);
            }
        }
    }
}
