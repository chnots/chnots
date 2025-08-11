use std::{
    cell::RefCell,
    collections::HashMap,
    ops::{Deref, DerefMut, Index},
};

use crate::krate::toent::logic::{
    EventBuilder, RawInputSegs, eventenum::EventEnum, timeevent::TimeEvent, todoevent::TodoEvent,
};
use chrono::NaiveDateTime;
use comrak::{
    arena_tree::Node,
    nodes::{Ast, LineColumn, NodeValue},
};
use itertools::Itertools;
use lazy_regex::Lazy;
use regex::Regex;

#[derive(Debug, Clone)]
enum ChnotBlockType {
    Heading,
    ListItem,
}

#[derive(Debug, Clone)]
pub enum PropsType {
    ID(String),
    ToentTransform {
        from_state: String,
        to_state: String,
        time: NaiveDateTime,
    },
}

enum InsertType {
    PropsType(PropsType),
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
pub struct ChnotBlock {
    title: String,
    block_type: ChnotBlockType,
    todo_event: Option<WithPos<TodoEvent>>,
    time_event: Vec<WithPos<TimeEvent>>,
    backlinks: Vec<WithPos<String>>,
    hashtags: Vec<WithPos<String>>,
    id: Option<WithPos<String>>,
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

static TOENT_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r"\{([^}]+)}");
static HASHTAG_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r"#([^\s#\[\]]+)");
static BACKLINK_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r"\[\[([0-9a-zA-Z]+)]]");

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
            todo_event: Vec<WithPos<TodoEvent>>,
            time_event: Vec<WithPos<TimeEvent>>,
            backlinks: Vec<WithPos<String>>,
            hashtags: Vec<WithPos<String>>,
            id: Vec<WithPos<String>>,
        }

        /// extract toent only from the listitem first line and headline
        fn extract_spans(
            text: &str,
            start_point: &Point,
            spans: &mut ChnotBlockGather,
            locater: &TextLocater<'_>,
        ) {
            for cps in TOENT_REGEX.captures_iter(text) {
                if let (Some(cp), Some(inner)) = (cps.get(0), cps.get(1)) {
                    let result = inner.as_str();
                    let start = start_point.offset + cp.start();
                    let end = start_point.offset + cp.end();
                    if let Ok(event) = EventEnum::try_from_standard(&RawInputSegs::from(result)) {
                        match event {
                            EventEnum::Time(time_event) => {
                                spans.time_event.push(WithPos {
                                    start_in: locater.locate_by_offset(start).unwrap(),
                                    end_ex: locater.locate_by_offset(end).unwrap(),
                                    data: *time_event,
                                    original: cp.as_str().to_string(),
                                });
                            }
                            EventEnum::Todo(todo_event) => {
                                spans.todo_event.push(WithPos {
                                    start_in: locater.locate_by_offset(start).unwrap(),
                                    end_ex: locater.locate_by_offset(end).unwrap(),
                                    data: todo_event,
                                    original: cp.as_str().to_string(),
                                });
                            }
                        }
                    }
                }
            }

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
                let trimmed = line.trim();
                let row = start_point.line + row;
                if let Some(id) = trimmed.strip_prefix("// ID: ") {
                    let line_end = start_point.column + line.len();
                    spans.id.push(WithPos {
                        start_in: locater.locate_by_linecol(LineColumn {
                            column: line_end - trimmed.len(),
                            line: row,
                        }),
                        end_ex: locater.locate_by_linecol(LineColumn {
                            column: line_end,
                            line: row,
                        }),
                        data: id.to_owned(),
                        original: trimmed.to_string(),
                    });
                }
            }
        }

        let locater: TextLocater<'_> = TextLocater::new(self.original);

        let mut starts: Vec<(ChnotBlockType, Point)> = vec![];
        let mut spans: ChnotBlockGather = ChnotBlockGather::default();

        for node in root.descendants() {
            let data = node.data.borrow();
            let pos = data.sourcepos;
            match &data.value {
                NodeValue::Item(_) => {
                    starts.push((
                        ChnotBlockType::ListItem,
                        locater.locate_by_linecol(pos.start),
                    ));
                }
                NodeValue::Heading(_) => {
                    starts.push((
                        ChnotBlockType::Heading,
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
            let todo_event = spans
                .todo_event
                .iter()
                .find(|s| s.start_in.offset >= start.offset && s.end_ex.offset < end_ex)
                .cloned();
            let id = spans
                .id
                .iter()
                .find(|s| s.start_in.offset >= start.offset && s.end_ex.offset < end_ex)
                .cloned();
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
                block_type: bt.to_owned(),
                todo_event,
                time_event: timeevents,
                backlinks,
                hashtags,
                end_ex,
                start: *start,
                id,
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

        if te.iter().any(|e| matches!(e, &TodoEvent::Doing)) {
            return Some(TodoEvent::Doing);
        }
        if te.iter().any(|e| matches!(e, &TodoEvent::Todo)) {
            return Some(TodoEvent::Todo);
        }

        if te.iter().any(|e| matches!(e, &TodoEvent::Wait)) {
            return Some(TodoEvent::Wait);
        }
        if te.iter().any(|e| matches!(e, &TodoEvent::Done)) {
            return Some(TodoEvent::Done);
        }

        if te.iter().any(|e| matches!(e, &TodoEvent::Cancel)) {
            return Some(TodoEvent::Cancel);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use comrak::nodes::LineColumn;

    use crate::krate::{
        chnot::parser::{ChnotParser, TextLocater},
        toent::logic::{EventBuilder, RawInputSegs, timeevent::TimeEvent, todoevent::TodoEvent},
    };

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
# ID、Hashtag 和 Toent #tagtest
// ID: file
Example Text

## {TODO} Head [[asd]] {1920-12-20 12:00:00}
// ID: head-foo

- List 1 {DONE} another {2025-06-26 11:19:26} [[123456789]] #hashtag1
    // ID: sdgdfgsdg
    // CLOSED: {2025-06-26 11:20:27}
    #tag2 [[418192012]] *italic* **bold** common text after bold
    what did you say? #tag3

    Paragraph 2 {DONE} 检查时间 {2025-06-26} [[34567890]]
    // ID: sdgdfgsdg1
    // CLOSED: {2025-06-26 11:20:27}
    ```txt
    {TODO} [[backlink-in-code]] #hashtagincode
    ```

    - {TODO} SUBITEM
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
    fn markdown_it() {
        let cp = ChnotParser::new(TEST_MARKDOWN);
        for cb in cp.chnot_map.values() {
            println!("> Source Begin ==========================");
            println!("{}", &TEST_MARKDOWN[cb.start.offset..cb.end_ex]);
            println!("> Source End ==========================");
            println!("-> BACKLINS");
            for ele in &cb.backlinks {
                let src = &TEST_MARKDOWN[ele.start_in.offset..ele.end_ex.offset];
                println!("TEXT: {}", &src);
                println!("PASR: {:?}", *ele);
                assert!(src == ele.original);
            }

            println!("-> HASHTAG");
            for ele in &cb.hashtags {
                let src = &TEST_MARKDOWN[ele.start_in.offset..ele.end_ex.offset];
                println!("TEXT: {}", &src);
                println!("PASR: {:?}", *ele);
                assert!(src == ele.original);
            }

            println!("-> TIMEVENT");
            for ele in &cb.time_event {
                let src: &str = &TEST_MARKDOWN[ele.start_in.offset..ele.end_ex.offset];
                println!("TEXT: {}", &src);
                println!("PASR: {:?}", *ele);

                assert!(src == ele.original);
            }

            println!("-> TODOEVENT");
            if let Some(ele) = &cb.todo_event {
                let src = &TEST_MARKDOWN[ele.start_in.offset..ele.end_ex.offset];
                println!("TEXT: {}", &src);
                println!("PASR: {:?}", *ele);

                assert!(src == ele.original);
            }

            println!("-> ID");
            if let Some(ele) = &cb.id {
                let src = &TEST_MARKDOWN[ele.start_in.offset..ele.end_ex.offset];
                println!("TEXT: {}", &src);
                println!("PASR: {:?}", *ele);

                assert!(src == ele.original);
            }
        }
    }
}
