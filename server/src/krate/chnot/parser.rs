use crate::krate::toent::logic::{timeevent::TimeEvent, todoevent::TodoEvent};
use markdown_it::{parser::inline::{InlineRule, InlineState}, Node};

pub struct ChnotBlock<'a> {
    id: &'a str,
    title: &'a str,
    todo_event: Option<TodoEvent>,
    time_event: Vec<TimeEvent>,
}

pub struct ToentBlock;
pub struct ToentBlockScanner;

pub struct PropertiesBlock;

pub struct ChnotParser {
    original: String,
}

impl ChnotParser {
    pub fn new(text: String) -> Self {
        Self { original: text }
    }
}
