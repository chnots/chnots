pub trait StringUtils {
    fn prefix(&self, prefix: &str) -> String;

    fn prefix_with_sep(&self, prefix: &str, sep: &str) -> String;
}

impl StringUtils for &str {
    fn prefix(&self, prefix: &str) -> String {
        format!("{}{}", prefix, self)
    }

    fn prefix_with_sep(&self, prefix: &str, sep: &str) -> String {
        format!("{}{}{}", prefix, sep, self)
    }
}
