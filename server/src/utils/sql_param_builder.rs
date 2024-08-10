use regex::Regex;

pub const MAGIC_SQL_PH: &'static str = "<CMGC-SPH>";

pub fn extract_magic_sql_ph(sql: &str) -> String {
    let re = Regex::new(MAGIC_SQL_PH).unwrap();
    let mut counter = 1;

    let result = re.replace_all(sql, |_: &regex::Captures| {
        let replacement = format!("${}", counter);
        counter += 1;
        replacement
    });

    result.to_string()
}

pub enum SqlParamValue {
    Str(String),
    Num(i64),
    VecStr(Vec<String>),
    VecNum(Vec<i64>),
    ILike(String),
    Fixed,
}

impl From<String> for SqlParamValue {
    fn from(value: String) -> Self {
        Self::Str(value)
    }
}

impl From<i64> for SqlParamValue {
    fn from(value: i64) -> Self {
        Self::Num(value)
    }
}

impl From<Vec<String>> for SqlParamValue {
    fn from(value: Vec<String>) -> Self {
        Self::VecStr(value)
    }
}

impl From<Vec<i64>> for SqlParamValue {
    fn from(value: Vec<i64>) -> Self {
        Self::VecNum(value)
    }
}

pub struct SqlParamBuilder {
    value_map: Vec<(String, SqlParamValue)>,
}

impl SqlParamBuilder {
    pub fn new() -> Self {
        Self {
            value_map: Vec::new(),
        }
    }

    pub fn with<T: Into<SqlParamValue>>(mut self, key: &str, value: T) -> Self {
        self.value_map.push((key.into(), value.into()));
        self
    }

    pub fn ilike<T: Into<String>>(mut self, key: &str, value: T) -> Self {
        self.value_map
            .push((key.into(), SqlParamValue::ILike(value.into())));
        self
    }

    pub fn fixed<E: Into<String>>(mut self, key: E) -> Self {
        self.value_map.push((key.into(), SqlParamValue::Fixed));
        self
    }

    pub fn build(self) -> (String, Vec<String>) {
        let mut sql = String::new();
        let mut values = vec![];

        let mut and = false;

        for (key, value) in self.value_map.into_iter() {
            match value {
                SqlParamValue::Str(v) => {
                    if and {
                        sql.push_str(" and ");
                    } else {
                        and = true;
                    }

                    sql.push_str(&key);
                    sql.push_str(" = $");
                    sql.push_str(&v);
                    values.push(v);
                }
                SqlParamValue::Num(v) => {
                    if and {
                        sql.push_str(" and ");
                    } else {
                        and = true;
                    }
                    values.push(v.to_string());
                    sql.push_str(&key);
                    sql.push_str(" = ");
                    sql.push_str(v.to_string().as_str());
                }
                SqlParamValue::VecStr(v) => {
                    if and {
                        sql.push_str(" and ");
                    } else {
                        and = true;
                    }

                    sql.push_str(&key);
                    sql.push_str(" in (");
                    let vs: Vec<&str> = v.iter().map(|_| MAGIC_SQL_PH).collect();
                    sql.push_str(vs.join(", ").as_str());
                    values.extend(v.into_iter());
                    sql.push(')')
                }
                SqlParamValue::VecNum(v) => {
                    if and {
                        sql.push_str(" and ");
                    } else {
                        and = true;
                    }

                    sql.push_str(&key);
                    sql.push_str(" in (");
                    sql.push_str(
                        v.iter()
                            .map(|e| e.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                            .as_str(),
                    );
                    sql.push(')')
                }
                SqlParamValue::ILike(v) => {
                    if and {
                        sql.push_str(" and ");
                    } else {
                        and = true;
                    }

                    sql.push_str(&key);
                    sql.push_str(" ilike $");
                    sql.push_str(&v);
                    values.push(format!("%{}%", v));
                }
                SqlParamValue::Fixed => {
                    sql.push_str(&key);
                }
            }
        }

        (sql, values)
    }
}
