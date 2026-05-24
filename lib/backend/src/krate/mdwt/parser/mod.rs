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
    pub content: String,
    pub order: usize,
    pub title: String,
}

static HEADING_OTID_CAPTURE_RE: Lazy<Regex> =
    lazy_regex::lazy_regex!(r"^(\s{0,3}#{1,6}\s+)\[\[(\d{13,20})\]\]\s*(.*)");

static HEADING_PLAIN_RE: Lazy<Regex> = lazy_regex::lazy_regex!(r"^(\s{0,3}#{1,6}\s+)(.*)");

pub fn parse_content_into_blocks(content: &str) -> (Vec<ParsedBlock>, String) {
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return (vec![], content.to_owned());
    }

    let mut heading_indices: Vec<usize> = vec![];
    for (i, line) in lines.iter().enumerate() {
        if HEADING_PLAIN_RE.is_match(line) {
            heading_indices.push(i);
        }
    }

    if heading_indices.is_empty() {
        return (
            vec![ParsedBlock {
                otid: TID::now(),
                content: content.to_owned(),
                order: 0,
                title: lines[0].to_owned(),
            }],
            content.to_owned(),
        );
    }

    let mut updated_lines: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
    let mut blocks: Vec<ParsedBlock> = Vec::with_capacity(heading_indices.len());

    for (block_idx, &start_line) in heading_indices.iter().enumerate() {
        let end_line = if block_idx + 1 < heading_indices.len() {
            heading_indices[block_idx + 1]
        } else {
            lines.len()
        };

        let heading_line = lines[start_line];
        let (otid, title, modified_heading) =
            if let Some(caps) = HEADING_OTID_CAPTURE_RE.captures(heading_line) {
                let prefix = caps.get(1).unwrap().as_str();
                let otid_str = caps.get(2).unwrap().as_str();
                let rest = caps.get(3).map(|m| m.as_str()).unwrap_or("");
                let otid: TID = otid_str
                    .parse::<i64>()
                    .ok()
                    .and_then(|v| TID::try_from(v).ok())
                    .unwrap_or_else(TID::now);
                (otid, rest.to_owned(), None)
            } else if let Some(caps) = HEADING_PLAIN_RE.captures(heading_line) {
                let prefix = caps.get(1).unwrap().as_str();
                let rest = caps.get(2).map(|m| m.as_str()).unwrap_or("");
                let otid = TID::now();
                let modified = format!("{}[[{}]] {}", prefix, otid, rest);
                (otid, rest.to_owned(), Some(modified))
            } else {
                (TID::now(), heading_line.to_owned(), None)
            };

        if let Some(ref new_heading) = modified_heading {
            updated_lines[start_line] = new_heading.clone();
        }

        let block_lines: Vec<&str> = if let Some(ref new_heading) = modified_heading {
            let mut bl: Vec<&str> = lines[start_line + 1..end_line].to_vec();
            bl
        } else {
            lines[start_line + 1..end_line].to_vec()
        };

        let mut content_parts = vec![];
        let heading_for_content = modified_heading.unwrap_or_else(|| heading_line.to_owned());
        content_parts.push(heading_for_content);
        for l in &block_lines {
            content_parts.push(l.to_string());
        }
        let block_content = content_parts.join("\n");

        blocks.push(ParsedBlock {
            otid,
            content: block_content,
            order: block_idx,
            title,
        });
    }

    (blocks, updated_lines.join("\n"))
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

    /// Helper: extract an OTID (13-20 digit number) from a `[[...]]` in a heading.
    fn extract_otid_from_heading(line: &str) -> Option<String> {
        lazy_regex::regex!(r"\[\[(\d{13,20})\]\]")
            .captures(line)
            .map(|c| c.get(1).unwrap().as_str().to_owned())
    }

    // --- Scenario 1: Empty content ---

    #[test]
    fn parse_empty_content_returns_no_blocks() {
        let (blocks, updated) = parse_content_into_blocks("");
        assert!(blocks.is_empty());
        assert_eq!(updated, "");
    }

    // --- Scenario 2: No heading — whole content as single block ---

    #[test]
    fn parse_no_heading_creates_single_block() {
        let content = "Just some plain text\nwithout any headings";
        let (blocks, updated) = parse_content_into_blocks(content);

        assert_eq!(blocks.len(), 1);
        let block = &blocks[0];
        assert_eq!(block.order, 0);
        assert_eq!(block.title, "Just some plain text");
        assert_eq!(block.content, content);
        // No OTID injection needed for non-heading content
        assert_eq!(updated, content);
    }

    #[test]
    fn parse_single_line_no_heading_creates_single_block() {
        let content = "A single line";
        let (blocks, _) = parse_content_into_blocks(content);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].title, "A single line");
    }

    // --- Scenario 3: Single heading → single block with auto OTID ---

    #[test]
    fn parse_single_heading_auto_otid() {
        let content = "## My First Block\nSome body text\nMore text";
        let (blocks, updated) = parse_content_into_blocks(content);

        assert_eq!(blocks.len(), 1);
        let block = &blocks[0];
        assert_eq!(block.order, 0);
        assert_eq!(block.title, "My First Block");
        assert!(block.content.starts_with("## [["));
        assert!(block.content.contains("My First Block"));
        assert!(block.content.contains("Some body text"));

        // Updated content should have the injected OTID
        assert!(updated.contains("[["));
        assert_ne!(updated, content);
    }

    // --- Scenario 4: Multiple headings → multiple blocks ---

    #[test]
    fn parse_multiple_headings_creates_multiple_blocks() {
        let content = "## Block A\nContent A\n\n## Block B\nContent B\n\n## Block C\nContent C";
        let (blocks, updated) = parse_content_into_blocks(content);

        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[0].title, "Block A");
        assert_eq!(blocks[1].title, "Block B");
        assert_eq!(blocks[2].title, "Block C");

        assert_eq!(blocks[0].order, 0);
        assert_eq!(blocks[1].order, 1);
        assert_eq!(blocks[2].order, 2);

        // Each block should have a distinct OTID
        assert_ne!(blocks[0].otid, blocks[1].otid);
        assert_ne!(blocks[1].otid, blocks[2].otid);

        // Updated content should have OTIDs injected
        assert!(updated.contains("[["));
    }

    // --- Scenario 5: Heading with existing OTID preserved ---

    #[test]
    fn parse_heading_with_existing_otid_preserved() {
        let content = "## [[1234567890123456789]] Existing Block\nBody text";
        let (blocks, updated) = parse_content_into_blocks(content);

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].otid.to_string(), "1234567890123456789");
        assert_eq!(blocks[0].title, "Existing Block");
        // Content unchanged since OTID was already present
        assert_eq!(updated, content);
    }

    #[test]
    fn parse_multiple_headings_mixed_otid() {
        let content = "## [[1111111111111111111]] Has OTID\nBody1\n\n## No OTID\nBody2\n\n## [[3333333333333333333]] Another OTID\nBody3";
        let (blocks, updated) = parse_content_into_blocks(content);

        assert_eq!(blocks.len(), 3);
        // First block keeps its OTID
        assert_eq!(blocks[0].otid.to_string(), "1111111111111111111");
        assert_eq!(blocks[0].title, "Has OTID");
        // Second block gets auto-generated OTID
        assert!(!blocks[1].otid.to_string().is_empty());
        assert_eq!(blocks[1].title, "No OTID");
        // Third block keeps its OTID
        assert_eq!(blocks[2].otid.to_string(), "3333333333333333333");

        // Updated content should inject OTID only for the second heading
        assert!(updated.contains("[[1111111111111111111]]"));
        assert!(updated.contains("[[3333333333333333333]]"));
        // The auto-injected OTID should appear
        let injected =
            extract_otid_from_heading(updated.lines().find(|l| l.contains("No OTID")).unwrap());
        assert!(injected.is_some());
        assert_eq!(injected.unwrap(), blocks[1].otid.to_string());
    }

    // --- Scenario 6: Different heading levels (h1 to h6) ---

    #[test]
    fn parse_various_heading_levels() {
        let content = "# H1 Title\nh1 body\n\n## H2 Title\nh2 body\n\n### H3 Title\nh3 body\n\n#### H4 Title\nh4 body\n\n##### H5 Title\nh5 body\n\n###### H6 Title\nh6 body";
        let (blocks, _) = parse_content_into_blocks(content);

        assert_eq!(blocks.len(), 6);
        assert_eq!(blocks[0].title, "H1 Title");
        assert_eq!(blocks[1].title, "H2 Title");
        assert_eq!(blocks[2].title, "H3 Title");
        assert_eq!(blocks[3].title, "H4 Title");
        assert_eq!(blocks[4].title, "H5 Title");
        assert_eq!(blocks[5].title, "H6 Title");
    }

    // --- Scenario 7: Heading with leading spaces (up to 3 allowed by regex) ---

    #[test]
    fn parse_heading_with_leading_spaces() {
        let content = "  ## Indented Heading\nBody text";
        let (blocks, _) = parse_content_into_blocks(content);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].title, "Indented Heading");
    }

    // --- Scenario 8: Block content includes text between headings only ---

    #[test]
    fn parse_block_content_isolation() {
        let content = "## First\nLine1\nLine2\n\n## Second\nLine3\nLine4";
        let (blocks, _) = parse_content_into_blocks(content);

        assert_eq!(blocks.len(), 2);
        // First block should contain heading + its body only
        let first = &blocks[0].content;
        assert!(first.contains("First"));
        assert!(first.contains("Line1"));
        assert!(first.contains("Line2"));
        assert!(!first.contains("Second"));
        assert!(!first.contains("Line3"));

        // Second block should contain heading + its body only
        let second = &blocks[1].content;
        assert!(second.contains("Second"));
        assert!(second.contains("Line3"));
        assert!(second.contains("Line4"));
        assert!(!second.contains("First"));
        assert!(!second.contains("Line1"));
    }

    // --- Scenario 9: Heading with no body text ---

    #[test]
    fn parse_heading_with_no_body() {
        let content = "## Empty Block\n\n## Next Block\nHas body";
        let (blocks, _) = parse_content_into_blocks(content);

        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].title, "Empty Block");
        assert_eq!(blocks[1].title, "Next Block");
    }

    // --- Scenario 10: OTID injection format correctness ---

    #[test]
    fn parse_otid_injection_format() {
        let content = "## Test Title\nbody";
        let (blocks, updated) = parse_content_into_blocks(content);

        let updated_heading = updated.lines().next().unwrap();
        let injected = extract_otid_from_heading(updated_heading);
        assert!(
            injected.is_some(),
            "injected heading should contain [[<OTID>]]"
        );

        assert_eq!(injected.unwrap(), blocks[0].otid.to_string());
    }

    // --- Scenario 11: Heading with backlinks and hashtags in title ---

    #[test]
    fn parse_heading_with_backlink_and_hashtag() {
        let content = "## [[1234567890123456789]] My Title [[backlink]] #tag\nBody";
        let (blocks, updated) = parse_content_into_blocks(content);

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].otid.to_string(), "1234567890123456789");
        assert_eq!(blocks[0].title, "My Title [[backlink]] #tag");
        assert_eq!(updated, content);
    }

    // --- Scenario 12: Only headings, no body anywhere ---

    #[test]
    fn parse_only_headings() {
        let content = "## A\n## B\n## C";
        let (blocks, _) = parse_content_into_blocks(content);
        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[0].title, "A");
        assert_eq!(blocks[1].title, "B");
        assert_eq!(blocks[2].title, "C");
    }

    // --- Scenario 13: Existing OTID that is invalid gets replaced ---

    #[test]
    fn parse_invalid_otid_gets_replaced() {
        // OTID must be 13-20 digits; "abc" is invalid so parse falls back to TID::now()
        let content = "## [[abc]] Invalid OTID\nBody";
        let (blocks, _updated) = parse_content_into_blocks(content);

        assert_eq!(blocks.len(), 1);
        // The OTID should be auto-generated, not "abc"
        assert_ne!(blocks[0].otid.to_string(), "abc");
    }

    // --- Scenario 14: Round-trip — parsing updated content yields same blocks ---

    #[test]
    fn parse_roundtrip_preserves_blocks() {
        let content = "## Alpha\nA body\n\n## Beta\nB body";
        let (blocks1, updated) = parse_content_into_blocks(content);

        // Re-parse the updated content (which now has OTIDs injected)
        let (blocks2, updated2) = parse_content_into_blocks(&updated);

        // Should get same number of blocks with same OTIDs
        assert_eq!(blocks1.len(), blocks2.len());
        for (b1, b2) in blocks1.iter().zip(blocks2.iter()) {
            assert_eq!(b1.otid, b2.otid, "OTIDs should be stable across re-parse");
            assert_eq!(b1.title, b2.title);
            assert_eq!(b1.order, b2.order);
        }

        // Second parse should not change content further
        assert_eq!(updated, updated2);
    }

    // --- Scenario 15: Multi-line content between headings with blank lines ---

    #[test]
    fn parse_blocks_with_blank_lines_and_code() {
        let content = "## Code Block\n```rust\nfn main() {}\n```\n\n## Text After\nSome text";
        let (blocks, _) = parse_content_into_blocks(content);

        assert_eq!(blocks.len(), 2);
        assert!(blocks[0].content.contains("```rust"));
        assert!(blocks[0].content.contains("fn main()"));
        assert!(blocks[1].content.contains("Some text"));
        // Code block should stay in first block, not leak to second
        assert!(!blocks[1].content.contains("fn main()"));
    }

    // --- Scenario 16: Heading with todo event markers ---

    #[test]
    fn parse_heading_with_todo_state() {
        let content = "## [TODO] Task Block\nDo something\n\n## [DONE] Completed Block\nDone";
        let (blocks, _) = parse_content_into_blocks(content);

        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].title, "[TODO] Task Block");
        assert_eq!(blocks[1].title, "[DONE] Completed Block");
    }

    // --- Scenario 17: Content with trailing newlines ---

    #[test]
    fn parse_content_with_trailing_newlines() {
        let content = "## Title\nBody\n\n\n\n";
        let (blocks, _) = parse_content_into_blocks(content);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].title, "Title");
    }
}
