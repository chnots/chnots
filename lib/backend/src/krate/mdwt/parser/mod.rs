use std::{
    cell::RefCell,
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::krate::toent::logic::{EventBuilder, timeevent::TimeEvent, todoevent::TodoEvent};
use chin_sql::time_type::TID;
use chrono::NaiveDateTime;
use comrak::{
    arena_tree::Node,
    nodes::{Ast, LineColumn, NodeValue},
};
use itertools::Itertools;
use lazy_regex::Lazy;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct ParsedBlock {
    pub otid: TID,
    /// Body-only content: for heading blocks, the text after stripping `## [[otid]] ` prefix
    /// (includes title text + body lines). For preamble blocks (level=0), the raw text.
    pub content: String,
    pub order: usize,
    /// First line of content (title text for heading blocks, empty for preamble)
    pub title: String,
    /// Heading level: 0 = preamble, 1-6 = heading level
    pub level: u8,
}

static HEADING_OTID_CAPTURE_RE: Lazy<Regex> =
    lazy_regex::lazy_regex!(r"^(\s{0,3}#{1,6}\s+)\[\[(\d{13,20})\]\]\s*(.*)");

static HEADING_PLAIN_RE: Lazy<Regex> = lazy_regex::lazy_regex!(r"^(\s{0,3}#{1,6}\s+)(.*)");

pub fn parse_content_into_blocks(
    content: &str,
    thread_otid: TID,
) -> anyhow::Result<Vec<ParsedBlock>> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return Ok(vec![]);
    }

    let mut heading_indices: Vec<usize> = vec![];
    for (i, line) in lines.iter().enumerate() {
        if HEADING_PLAIN_RE.is_match(line) {
            heading_indices.push(i);
        }
    }

    if heading_indices.is_empty() {
        // No headings — entire content is preamble if non-empty
        if content.trim().is_empty() {
            return Ok(vec![]);
        }
        return Ok(vec![ParsedBlock {
            otid: thread_otid,
            content: content.to_owned(),
            order: 0,
            title: String::new(),
            level: 0,
        }]);
    }

    let mut blocks: Vec<ParsedBlock> = Vec::with_capacity(heading_indices.len() + 1);
    let mut order = 0;

    // Check for preamble (non-empty content before first heading)
    if heading_indices[0] > 0 {
        let preamble_lines = &lines[0..heading_indices[0]];
        let preamble_text = preamble_lines.join("\n");
        if !preamble_text.trim().is_empty() {
            blocks.push(ParsedBlock {
                otid: thread_otid,
                content: preamble_text,
                order,
                title: String::new(),
                level: 0,
            });
            order += 1;
        }
    }

    for (block_idx, &start_line) in heading_indices.iter().enumerate() {
        let end_line = if block_idx + 1 < heading_indices.len() {
            heading_indices[block_idx + 1]
        } else {
            lines.len()
        };

        let heading_line = lines[start_line];
        let (otid, title, level) =
            if let Some(caps) = HEADING_OTID_CAPTURE_RE.captures(heading_line) {
                let otid_str = caps.get(2).unwrap().as_str();
                let rest = caps.get(3).map(|m| m.as_str()).unwrap_or("");
                let otid: TID = otid_str
                    .parse::<i64>()
                    .ok()
                    .and_then(|v| TID::try_from(v).ok())
                    .ok_or_else(|| anyhow::anyhow!("invalid OTID in heading: {}", otid_str))?;
                let prefix = caps.get(1).unwrap().as_str();
                let level = prefix
                    .trim_start()
                    .chars()
                    .take_while(|c| *c == '#')
                    .count() as u8;
                (otid, rest.to_owned(), level)
            } else {
                let title = HEADING_PLAIN_RE
                    .captures(heading_line)
                    .and_then(|c| c.get(2))
                    .map(|m| m.as_str().to_owned())
                    .unwrap_or_else(|| heading_line.to_owned());
                anyhow::bail!("heading '{}' is missing OTID", title);
            };

        // Body-only: title text + remaining lines (strip the `## [[otid]] ` prefix)
        let block_lines = &lines[start_line + 1..end_line];
        let block_content = if block_lines.is_empty() {
            title.clone()
        } else {
            [&title as &str]
                .into_iter()
                .chain(block_lines.iter().copied())
                .join("\n")
        };

        blocks.push(ParsedBlock {
            otid,
            content: block_content,
            order,
            title,
            level,
        });
        order += 1;
    }

    Ok(blocks)
}

#[derive(Debug, Clone)]
enum ChnotBlockEnum {
    Heading,
    ListItem,
}

#[derive(Debug, Clone)]
enum PropsEnum {
    ID(String),
    ToentDefi(TimeEvent),
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

pub struct MdwtParser<'a> {
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
static EVENT_PROP_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r"(?i)^\s*;+\s*EVENT:\s*(.*?)\s*$");

static HASHTAG_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r"#([^\s#\[\]]+)");
static BACKLINK_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r"\[\[([0-9a-zA-Z]+)]]");

#[derive(Debug, Clone)]
struct Props {
    key: String,
    value: String,
}

impl<'a> MdwtParser<'a> {
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
            timeevents: Vec<WithPos<TimeEvent>>,
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
                .timeevents
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
        self.chnot_map
            .values()
            .filter_map(|c| c.todo_event.as_ref())
            .min_by_key(|te| te.start_in.offset)
            .map(|te| te.data)
    }

    pub fn get_outer_time_events(&self) -> Vec<TimeEvent> {
        self.original
            .lines()
            .filter_map(|line| {
                let caps = EVENT_PROP_REGEX.captures(line)?;
                let raw = caps.get(1)?.as_str().trim();
                TimeEvent::try_from_standrd_str(raw).ok()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use comrak::nodes::LineColumn;

    use crate::krate::mdwt::parser::{MdwtParser, TextLocater};

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
        let cp = MdwtParser::new(TEST_MARKDOWN);
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

    // ============================================================
    // Tests for parse_content_into_blocks
    // ============================================================

    use super::parse_content_into_blocks;
    use chin_sql::time_type::TID;

    fn tid(s: i64) -> TID {
        TID::try_from(s).unwrap()
    }

    const THREAD_OTID: i64 = 1748000000000000;

    // --- Error cases ---

    #[test]
    fn parse_empty_content_returns_no_blocks() {
        let blocks = parse_content_into_blocks("", tid(THREAD_OTID)).unwrap();
        assert!(blocks.is_empty());
    }

    #[test]
    fn parse_heading_without_otid_returns_error() {
        let result = parse_content_into_blocks("## No OTID Here\nBody text", tid(THREAD_OTID));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("missing OTID"));
    }

    #[test]
    fn parse_invalid_otid_returns_error() {
        let result = parse_content_into_blocks(
            "## [[9999999999999999999]] Out Of Range\nBody",
            tid(THREAD_OTID),
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("invalid OTID"));
    }

    #[test]
    fn parse_mixed_headings_missing_otid_returns_error() {
        let content = "## [[1000000000000000]] Has OTID\nBody1\n\n## No OTID\nBody2";
        let result = parse_content_into_blocks(content, tid(THREAD_OTID));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("missing OTID"));
    }

    // --- Preamble cases ---

    #[test]
    fn parse_no_heading_with_content_creates_preamble() {
        let content = "Just some plain text\nwithout any headings";
        let blocks = parse_content_into_blocks(content, tid(THREAD_OTID)).unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].otid, tid(THREAD_OTID));
        assert_eq!(blocks[0].level, 0);
        assert_eq!(blocks[0].content, content);
        assert_eq!(blocks[0].title, "");
    }

    #[test]
    fn parse_preamble_before_headings() {
        let content = "Hello world\n\n## [[1000000000000000]] First Heading\nBody text";
        let blocks = parse_content_into_blocks(content, tid(THREAD_OTID)).unwrap();
        assert_eq!(blocks.len(), 2);

        // Preamble
        assert_eq!(blocks[0].otid, tid(THREAD_OTID));
        assert_eq!(blocks[0].level, 0);
        assert_eq!(blocks[0].content, "Hello world\n");
        assert_eq!(blocks[0].order, 0);

        // Heading block
        assert_eq!(blocks[1].otid.to_string(), "1000000000000000");
        assert_eq!(blocks[1].level, 2);
        assert_eq!(blocks[1].title, "First Heading");
        assert_eq!(blocks[1].order, 1);
    }

    #[test]
    fn parse_whitespace_preamble_not_created() {
        let content = "   \n  \n## [[1000000000000000]] Heading\nBody";
        let blocks = parse_content_into_blocks(content, tid(THREAD_OTID)).unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].level, 2);
        assert_eq!(blocks[0].title, "Heading");
    }

    // --- Valid cases with OTIDs (body-only content) ---

    #[test]
    fn parse_single_heading_with_otid() {
        let content = "## [[1000000000000000]] My First Block\nSome body text\nMore text";
        let blocks = parse_content_into_blocks(content, tid(THREAD_OTID)).unwrap();

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].otid.to_string(), "1000000000000000");
        assert_eq!(blocks[0].title, "My First Block");
        assert_eq!(blocks[0].level, 2);
        // Body-only: title + body lines, no heading prefix
        assert_eq!(
            blocks[0].content,
            "My First Block\nSome body text\nMore text"
        );
    }

    #[test]
    fn parse_multiple_headings_with_otids() {
        let content = "## [[1000000000000000]] Block A\nContent A\n\n## [[2000000000000000]] Block B\nContent B\n\n## [[3000000000000000]] Block C\nContent C";
        let blocks = parse_content_into_blocks(content, tid(THREAD_OTID)).unwrap();

        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[0].otid.to_string(), "1000000000000000");
        assert_eq!(blocks[0].title, "Block A");
        assert_eq!(blocks[0].level, 2);
        assert_eq!(blocks[1].otid.to_string(), "2000000000000000");
        assert_eq!(blocks[1].title, "Block B");
        assert_eq!(blocks[2].otid.to_string(), "3000000000000000");
        assert_eq!(blocks[2].title, "Block C");

        assert_eq!(blocks[0].order, 0);
        assert_eq!(blocks[1].order, 1);
        assert_eq!(blocks[2].order, 2);
    }

    #[test]
    fn parse_various_heading_levels() {
        let content = "# [[1000000000000000]] H1\nh1 body\n\n## [[2000000000000000]] H2\nh2 body\n\n### [[3000000000000000]] H3\nh3 body";
        let blocks = parse_content_into_blocks(content, tid(THREAD_OTID)).unwrap();

        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[0].level, 1);
        assert_eq!(blocks[0].title, "H1");
        assert_eq!(blocks[1].level, 2);
        assert_eq!(blocks[1].title, "H2");
        assert_eq!(blocks[2].level, 3);
        assert_eq!(blocks[2].title, "H3");
    }

    #[test]
    fn parse_heading_with_leading_spaces() {
        let content = "  ## [[1000000000000000]] Indented Heading\nBody text";
        let blocks = parse_content_into_blocks(content, tid(THREAD_OTID)).unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].title, "Indented Heading");
        assert_eq!(blocks[0].level, 2);
    }

    #[test]
    fn parse_block_content_isolation() {
        let content = "## [[1000000000000000]] First\nLine1\nLine2\n\n## [[2000000000000000]] Second\nLine3\nLine4";
        let blocks = parse_content_into_blocks(content, tid(THREAD_OTID)).unwrap();

        assert_eq!(blocks.len(), 2);
        assert!(blocks[0].content.contains("First"));
        assert!(blocks[0].content.contains("Line1"));
        assert!(!blocks[0].content.contains("Second"));
        assert!(!blocks[0].content.contains("##"));

        assert!(blocks[1].content.contains("Second"));
        assert!(blocks[1].content.contains("Line3"));
        assert!(!blocks[1].content.contains("First"));
        assert!(!blocks[1].content.contains("##"));
    }

    #[test]
    fn parse_heading_with_no_body() {
        let content = "## [[1000000000000000]] A\n\n## [[2000000000000000]] B\nHas body";
        let blocks = parse_content_into_blocks(content, tid(THREAD_OTID)).unwrap();

        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].title, "A");
        // Title + empty line between headings
        assert_eq!(blocks[0].content, "A\n");
        assert_eq!(blocks[1].title, "B");
    }

    #[test]
    fn parse_heading_with_backlink_and_hashtag() {
        let content = "## [[1000000000000000]] My Title [[backlink]] #tag\nBody";
        let blocks = parse_content_into_blocks(content, tid(THREAD_OTID)).unwrap();

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].otid.to_string(), "1000000000000000");
        assert_eq!(blocks[0].title, "My Title [[backlink]] #tag");
    }

    #[test]
    fn parse_roundtrip_preserves_blocks() {
        let content =
            "## [[1000000000000000]] Alpha\nA body\n\n## [[2000000000000000]] Beta\nB body";
        let blocks = parse_content_into_blocks(content, tid(THREAD_OTID)).unwrap();

        let blocks2 = parse_content_into_blocks(content, tid(THREAD_OTID)).unwrap();
        assert_eq!(blocks.len(), blocks2.len());
        for (b1, b2) in blocks.iter().zip(blocks2.iter()) {
            assert_eq!(b1.otid, b2.otid);
            assert_eq!(b1.title, b2.title);
            assert_eq!(b1.level, b2.level);
        }
    }

    #[test]
    fn parse_blocks_with_code() {
        let content = "## [[1000000000000000]] Code Block\n```rust\nfn main() {}\n```\n\n## [[2000000000000000]] Text After\nSome text";
        let blocks = parse_content_into_blocks(content, tid(THREAD_OTID)).unwrap();

        assert_eq!(blocks.len(), 2);
        assert!(blocks[0].content.contains("```rust"));
        assert!(!blocks[1].content.contains("fn main()"));
    }

    #[test]
    fn parse_content_with_trailing_newlines() {
        let content = "## [[1000000000000000]] Title\nBody\n\n\n\n";
        let blocks = parse_content_into_blocks(content, tid(THREAD_OTID)).unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].title, "Title");
    }

    #[test]
    fn parse_content_not_starting_with_heading() {
        // Empty trimmed content before heading → no preamble
        let content = "\n## [[1000000000000000]] Title\nBody";
        let blocks = parse_content_into_blocks(content, tid(THREAD_OTID)).unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].level, 2);
    }
}
