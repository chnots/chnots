use postgres_types::ToSql;
use regex::Regex;
use tracing::info;

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

enum SqlParamValue {
    WhereEqual(Box<dyn ToSql + Sync + Send>),
    WhereIn(Vec<Box<dyn ToSql + Sync + Send>>),
    WhereILike(String),
    WhereIsNull(bool),
    Raw(Vec<Box<dyn ToSql + Sync + Send>>),
    Limit { limit: i64, offset: i64 },
    Fixed,
}

pub struct PgParamBuilder {
    segs: Vec<(String, SqlParamValue)>,
}

impl PgParamBuilder {
    pub fn new<T: Into<String>>(sql: T) -> Self {
        Self {
            segs: vec![(sql.into(), SqlParamValue::Fixed)],
        }
    }

    pub fn where_in<T: Into<Box<dyn ToSql + Sync + Send>>>(
        mut self,
        key: &str,
        value: Vec<T>,
    ) -> Self {
        self.segs.push((
            key.into(),
            SqlParamValue::WhereIn(value.into_iter().map(|e| e.into()).collect()),
        ));
        self
    }

    pub fn where_equal<T: ToSql + Sync + Send + 'static>(mut self, key: &str, value: T) -> Self {
        self.segs
            .push((key.into(), SqlParamValue::WhereEqual(Box::new(value))));
        self
    }

    pub fn where_ilike<T: Into<String>>(mut self, key: &str, value: T) -> Self {
        self.segs
            .push((key.into(), SqlParamValue::WhereILike(value.into())));
        self
    }

    pub fn where_null(mut self, key: &str, null: bool) -> Self {
        self.segs
            .push((key.into(), SqlParamValue::WhereIsNull(null)));
        self
    }

    pub fn raw(mut self, key: &str, values: Vec<Box<dyn ToSql + Sync + Send>>) -> Self {
        self.segs.push((key.into(), SqlParamValue::Raw(values)));
        self
    }

    pub fn limit(mut self, key: &str, offset: i64, limit: i64) -> Self {
        self.segs
            .push((key.into(), SqlParamValue::Limit { limit, offset }));
        self
    }

    pub fn option_ilike<T: Into<String>>(mut self, key: &str, value: Option<T>) -> Self {
        if let Some(value) = value {
            self.segs
                .push((key.into(), SqlParamValue::WhereILike(value.into())));
        }
        self
    }

    pub fn option_equal<T: Into<Box<dyn ToSql + Sync + Send>>>(
        mut self,
        key: &str,
        value: Option<T>,
    ) -> Self {
        if let Some(value) = value {
            self.segs
                .push((key.into(), SqlParamValue::WhereEqual(value.into())));
        }
        self
    }

    pub fn option_set<T: Into<Box<dyn ToSql + Sync + Send>>>(
        mut self,
        key: &str,
        value: Option<T>,
    ) -> Self {
        if let Some(value) = value {
            self.segs
                .push((key.into(), SqlParamValue::WhereEqual(value.into())));
        }
        self
    }

    pub fn fixed(mut self, key: &str) -> Self {
        self.segs.push((key.into(), SqlParamValue::Fixed));
        self
    }

    pub fn build(self) -> (String, Vec<Box<dyn ToSql + Sync + Send>>) {
        let re = Regex::new("\\{\\}").unwrap();

        let mut sql = String::new();
        let mut values = vec![];

        let mut ph = 1;
        let mut state = 0;

        macro_rules! cond_with {
            () => {
                if state == 0 {
                    sql.push_str(" where ");
                    state = 1;
                } else if state == 1 {
                    sql.push_str(" and ");
                }
            };
        }

        macro_rules! push_ph {
            () => {

                    sql.push('$');
                    sql.push_str(format!("{}", ph).as_str());
                    ph += 1

            };
        }

        for (key, value) in self.segs.into_iter() {
            match value {
                SqlParamValue::WhereEqual(r) => {
                    cond_with!();
                    sql.push('(');
                    sql.push_str(&key);
                    sql.push_str(" = ");
                    push_ph!();
                    sql.push(')');
                    values.push(r);
                }
                SqlParamValue::WhereIn(r) => {
                    cond_with!();
                    sql.push('(');
                    sql.push_str(&key);
                    sql.push_str(" in ");
                    sql.push('(');
                    for _ in 0..r.len().saturating_sub(1) {
                        push_ph!();
                        sql.push(',');
                    }
                    push_ph!();
                    sql.push(')');
                    sql.push(')');
                    values.extend(r);
                }
                SqlParamValue::WhereILike(r) => {
                    cond_with!();
                    sql.push('(');
                    sql.push_str(&key);
                    sql.push_str(" ilike ");
                    push_ph!();
                    sql.push(')');
                    values.push(Box::new(format!("%{}%", r)));
                }
                SqlParamValue::WhereIsNull(r) => {
                    cond_with!();
                    sql.push('(');
                    sql.push_str(&key);
                    sql.push_str(" is ");

                    if r {
                        sql.push_str("null ")
                    } else {
                        sql.push_str("not null ");
                    }

                    sql.push(')');
                }
                SqlParamValue::Raw(r) => {
                    let result = re.replace_all(&key, |_: &regex::Captures| {
                        let replacement = format!("${}", ph);
                        ph += 1;
                        replacement
                    });
                    sql.push_str(result.as_ref());
                    values.extend(r);
                }
                SqlParamValue::Fixed => {
                    sql.push_str(&key);
                }
                SqlParamValue::Limit { limit, offset } => {
                    sql.push_str(" limit ");
                    push_ph!();
                    sql.push_str(" offset ");
                    push_ph!();
                    values.push(Box::new(limit));
                    values.push(Box::new(offset));
                }
            }
        }

        info!("sql is {}, {:?}", sql, values);
        (sql, values)
    }
}
