
use strum::{AsRefStr, EnumIter, EnumString};

#[derive(Clone, Debug, EnumString, AsRefStr, EnumIter)]
#[strum(serialize_all = "UPPERCASE")]
pub enum TodoEvent {
    Todo,
    Doing,
    Wait,
    Done,
    Cancel,
}

impl Into<String> for TodoEvent {
    fn into(self) -> String {
        return self.as_ref().to_string();
    }
}