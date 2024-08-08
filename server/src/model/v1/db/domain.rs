use std::borrow::Cow;

pub struct Domain {
    pub manager: Cow<'static, str>,
    pub name: Cow<'static, str>,
}
