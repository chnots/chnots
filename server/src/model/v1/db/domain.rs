use std::borrow::Cow;

pub struct Domain {
    pub manager: Vec<Cow<'static, str>>,
    pub name: Cow<'static, str>,
}
