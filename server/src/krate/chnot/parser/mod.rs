use std::collections::HashMap;

use crate::krate::toent::logic::{
    EventBuilder, RawInputSegs, eventenum::EventEnum, timeevent::TimeEvent, todoevent::TodoEvent,
};
use chrono::NaiveDateTime;
use itertools::Itertools;
use lazy_regex::Lazy;
use log::info;
use markdown::mdast::{Node, Paragraph};
use regex::Regex;

#[derive(Debug, Clone)]
enum ChnotBlockType {
    Heading {
        level: u8,
        pos: usize,
        row_num: usize,
    },
    ListItem {
        level: usize,
        pos: usize,
        row_num: usize,
    },
}

impl ChnotBlockType {
    fn get_row_num(&self) -> usize {
        match self {
            ChnotBlockType::Heading {
                level: _,
                pos: _,
                row_num,
            } => *row_num,
            ChnotBlockType::ListItem {
                level: _,
                pos: _,
                row_num,
            } => *row_num,
        }
    }
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

#[derive(Debug, Clone)]
pub struct ChnotBlock {
    title: String,
    block_type: ChnotBlockType,
    todo_event: Option<TodoEvent>,
    time_event: Vec<TimeEvent>,
    backlinks: Vec<String>,
    hashtags: Vec<String>,
    props: Vec<PropsType>,
}

pub struct ChnotParser<'a> {
    original: &'a str,
    new_map: HashMap<usize, InsertType>,
    chnot_map: HashMap<usize, ChnotBlock>,
    root: Node,
}

static TOENT_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r"\{([^}]+)}");
static HASHTAG_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r"#([^\s#\[\]]+)");
static BACKLINK_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r"\[\[([0-9a-zA-Z]+)]]");

impl<'a> ChnotParser<'a> {
    pub fn new(text: &'a str) -> Self {
        let ast = markdown::to_mdast(text, &markdown::ParseOptions::gfm()).unwrap();

        Self {
            original: text,
            new_map: Default::default(),
            chnot_map: Default::default(),
            root: ast,
        }
    }

    pub fn parse(&mut self) {
        /// extract toent only from the listitem first line and headline
        fn extract_toent(node: &Node, chnot_block: &mut ChnotBlock) -> bool {
            match node {
                Node::Text(text) => {
                    for cps in TOENT_REGEX.captures_iter(&text.value) {
                        if let Some(cp) = cps.get(1) {
                            let result = cp.as_str();
                            if let Ok(event) =
                                EventEnum::try_from_standard(&RawInputSegs::from(result))
                            {
                                match event {
                                    EventEnum::Time(time_event) => {
                                        chnot_block.time_event.push(*time_event);
                                    }
                                    EventEnum::Todo(todo_event) => {
                                        chnot_block.todo_event.replace(todo_event);
                                    }
                                }
                            }
                        }
                    }
                    if text.value.contains("\n") {
                        return true;
                    }
                }
                Node::Paragraph(paragraph) => {
                    for s in &paragraph.children {
                        match s {
                            Node::Break(_) => {
                                break;
                            }
                            node => {
                                if extract_toent(node, chnot_block) {
                                    break;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            false
        }

        fn extract_backlink_and_hashtags(node: &Node, chnot_block: &mut ChnotBlock) {
            match node {
                Node::Text(text) => {
                    for cps in HASHTAG_REGEX.captures_iter(&text.value) {
                        if let Some(cp) = cps.get(0) {
                            let result = cp.as_str();
                            chnot_block.hashtags.push(result.to_string());
                        }
                    }

                    for cps in BACKLINK_REGEX.captures_iter(&text.value) {
                        if let Some(cp) = cps.get(1) {
                            let result = cp.as_str();
                            chnot_block.backlinks.push(result.to_string());
                        }
                    }
                }
                Node::Paragraph(paragraph) => {
                    for s in &paragraph.children {
                        match s {
                            Node::Break(_) => {
                                break;
                            }
                            node => {
                                extract_backlink_and_hashtags(node, chnot_block);
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // parse the first pargraph of headline or listitem
        fn extract_props_from_para(para: &Paragraph, chnot_block: &mut ChnotBlock, original: &str) {
            if let Some(pos) = &para.position {
                for p in original[pos.start.offset..pos.end.offset].split("\n") {
                    let p = p.trim_start();
                    let prefix = "// ID: ";
                    if p.starts_with(prefix) {
                        chnot_block
                            .props
                            .push(PropsType::ID(p[prefix.len()..].to_string()));
                    }
                }
            }
        }

        fn walk(this: &mut ChnotParser<'_>, node: &Node) {
            match node {
                Node::Heading(heading) => {
                    if let Some(pos) = &heading.position {
                        let mut cb = ChnotBlock {
                            title: this.original[pos.start.offset..pos.end.offset].to_string(),
                            block_type: ChnotBlockType::Heading {
                                level: heading.depth,
                                pos: pos.start.offset,
                                row_num: pos.start.line,
                            },
                            todo_event: None,
                            time_event: vec![],
                            backlinks: vec![],
                            props: vec![],
                            hashtags: vec![],
                        };

                        // extract
                        for n in &heading.children {
                            extract_toent(n, &mut cb);
                            extract_backlink_and_hashtags(n, &mut cb);
                        }

                        this.chnot_map.insert(pos.start.offset, cb);
                    }
                }
                Node::ListItem(list_item) => {
                    if let Some(pos) = &list_item.position {
                        let fline = &this.original[pos.start.offset..pos.end.offset];
                        let fline = fline
                            .split_once('\n')
                            .map_or(fline, |(first, _)| first)
                            .to_string();

                        let mut cb = ChnotBlock {
                            title: fline,
                            block_type: ChnotBlockType::ListItem {
                                level: pos.start.column,
                                pos: pos.start.offset,
                                row_num: pos.start.line,
                            },
                            todo_event: None,
                            time_event: vec![],
                            backlinks: vec![],
                            props: vec![],
                            hashtags: vec![],
                        };

                        // extract
                        if let Some(first) = list_item.children.first() {
                            extract_toent(first, &mut cb);
                            if let Node::Paragraph(para) = first {
                                extract_props_from_para(para, &mut cb, this.original);
                            }
                        }
                        for n in &list_item.children {
                            extract_backlink_and_hashtags(n, &mut cb);
                        }

                        this.chnot_map.insert(pos.start.offset, cb);
                    }
                }
                Node::Table(table) => {
                    for li in &table.children {
                        walk(this, li);
                    }
                }
                Node::List(list) => {
                    for li in &list.children {
                        walk(this, li);
                    }
                }
                Node::TableRow(table_row) => {
                    for li in &table_row.children {
                        walk(this, li);
                    }
                }
                Node::TableCell(table_cell) => {
                    for li in &table_cell.children {
                        walk(this, li);
                    }
                }
                Node::Paragraph(paragraph) => {
                    if let Some(pos) = &paragraph.position {
                        if let Some((_, cb)) = this
                            .chnot_map
                            .iter_mut()
                            .find(|(_, cb)| cb.block_type.get_row_num() == pos.start.line - 1)
                        {
                            extract_props_from_para(paragraph, cb, this.original);
                        }
                    }
                    for ele in &paragraph.children {
                        walk(this, ele);
                    }
                }
                Node::Text(text) => {
                    if let Some(pos) = &text.position {
                        if let Some((_, c)) = this
                            .chnot_map
                            .values_mut()
                            .map(|cb| (pos.start.line - cb.block_type.get_row_num(), cb))
                            .filter(|(diff, _)| *diff > 0)
                            .sorted_by(|c1, c2| c1.0.cmp(&c2.0))
                            .take(0)
                            .last()
                        {
                            extract_backlink_and_hashtags(node, c);
                        }
                    }
                }
                _ => {}
            }
        }

        let root = self.root.clone();
        if let Some(ns) = root.children() {
            for node in ns {
                walk(self, node);
            }
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
            .filter_map(|c| c.todo_event)
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
    use itertools::Itertools;

    use crate::krate::chnot::parser::ChnotParser;

    const TEST_MARKDOWN: &str = "# ID、Hashtag 和 Toent #tagtest
// ID:a12cd


下面是一个例子

## {TODO} Foo Title [[asd]] {1920-12-20 12:00:00}
// ID: a12cd

  - Paragraph 1 {DONE} 检查时间 {2025-06-26 11:19:26} [[123456789]] #haslll
    // ID: sdgdfgsdg
    // CLOSED: {2025-06-26 11:20:27}

    Paragraph 2 {DONEasdfl} 检查时间 {2025-06-26} [[34567890]]
    // ID: sdgdfgsdg1
    // CLOSED: {2025-06-26 11:20:27}
    - SUBITEM
    ";

    #[test]
    fn markdown_it() {
        let mut v = ChnotParser::new(TEST_MARKDOWN);
        v.parse();
        for ele in v.chnot_map.iter().sorted_by(|e, v| e.0.cmp(v.0)) {
            println!("{ele:?}")
        }
    }
}
