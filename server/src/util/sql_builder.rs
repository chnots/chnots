use std::borrow::Cow;

use chrono::{DateTime, FixedOffset};

pub enum SqlValue<'a> {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Str(Cow<'a, str>),
    Date(Cow<'a, DateTime<FixedOffset>>),
    Bool(bool),
    Opt(Option<Box<SqlValue<'a>>>),
}

impl<'a> Into<SqlValue<'a>> for i8 {
    fn into(self) -> SqlValue<'a> {
        SqlValue::I8(self)
    }
}

impl<'a> Into<SqlValue<'a>> for i16 {
    fn into(self) -> SqlValue<'a> {
        SqlValue::I16(self)
    }
}

impl<'a> Into<SqlValue<'a>> for i32 {
    fn into(self) -> SqlValue<'a> {
        SqlValue::I32(self)
    }
}

impl<'a> Into<SqlValue<'a>> for i64 {
    fn into(self) -> SqlValue<'a> {
        SqlValue::I64(self)
    }
}

impl<'a> Into<SqlValue<'a>> for &'a String {
    fn into(self) -> SqlValue<'a> {
        SqlValue::Str(Cow::Borrowed(self))
    }
}

impl<'a> Into<SqlValue<'a>> for &'a str {
    fn into(self) -> SqlValue<'a> {
        SqlValue::Str(Cow::Borrowed(self))
    }
}

impl<'a> Into<SqlValue<'a>> for String {
    fn into(self) -> SqlValue<'a> {
        SqlValue::Str(Cow::Owned(self))
    }
}

impl<'a> Into<SqlValue<'a>> for Cow<'a, str> {
    fn into(self) -> SqlValue<'a> {
        SqlValue::Str(self)
    }
}

impl<'a> Into<SqlValue<'a>> for bool {
    fn into(self) -> SqlValue<'a> {
        SqlValue::Bool(self)
    }
}

impl<'a> Into<SqlValue<'a>> for DateTime<FixedOffset> {
    fn into(self) -> SqlValue<'a> {
        SqlValue::Date(Cow::Owned(self))
    }
}

impl<'a, T: Into<SqlValue<'a>>> Into<SqlValue<'a>> for Option<T> {
    fn into(self) -> SqlValue<'a> {
        SqlValue::Opt(self.map(|e| {
            let sv: SqlValue<'a> = e.into();
            sv.into()
        }))
    }
}

pub enum ValueType {
    QustionMark,
    DollarNumber(i32),
}

impl ValueType {
    pub fn dollar_number() -> Self {
        ValueType::DollarNumber(0)
    }

    pub fn question_mark() -> Self {
        ValueType::QustionMark
    }

    fn next(&mut self) -> String {
        match self {
            ValueType::QustionMark => "?".to_owned(),
            ValueType::DollarNumber(n) => {
                *n += 1;
                format!("${}", n)
            }
        }
    }
}

pub struct SqlSeg<'a> {
    pub seg: String,
    pub values: Vec<SqlValue<'a>>,
}

pub enum ConjOp {
    And,
    Or,
}

pub enum Wheres<'a> {
    Conj(ConjOp, Vec<Wheres<'a>>),
    In(&'a str, Vec<SqlValue<'a>>),
    Not(Box<Wheres<'a>>),
    Compare(&'a str, &'a str, SqlValue<'a>), // key, operator, value
    None,
}

impl<'a> Wheres<'a> {
    pub fn equal<T: Into<SqlValue<'a>>>(key: &'a str, v: T) -> Self {
        Self::Compare(key, "=", v.into())
    }

    pub fn ilike<T: AsRef<str>>(key: &'a str, v: T) -> Self {
        Self::Compare(key, "ilike", format!("%{}%", v.as_ref()).into())
    }

    pub fn is_null(key: &'a str) -> Self {
        Self::Compare(key, "is", "null".into())
    }

    pub fn if_some<T, F>(original: Option<T>, map: F) -> Self
    where
        F: FnOnce(T) -> Self,
    {
        match original {
            Some(t) => map(t),
            None => Wheres::None,
        }
    }
    pub fn and<T: Into<Vec<Wheres<'a>>>>(values: T) -> Self {
        Self::Conj(ConjOp::And, values.into())
    }

    pub fn or<T: Into<Vec<Wheres<'a>>>>(values: T) -> Self {
        Self::Conj(ConjOp::Or, values.into())
    }

    pub fn transform<T, F>(original: T, map: F) -> Self
    where
        F: FnOnce(T) -> Self,
    {
        map(original)
    }

    pub fn is_in<T: Into<SqlValue<'a>>>(key: &'a str, values: Vec<T>) -> Self {
        let s = Self::In(key, values.into_iter().map(|e| e.into()).collect());
        s
    }

    pub fn build(self, value_type: &mut ValueType) -> Option<SqlSeg<'a>> {
        let mut seg = String::new();
        let mut values: Vec<SqlValue<'a>> = Vec::new();

        match self {
            Wheres::Conj(op, fs) => {
                let vs: Vec<String> = fs
                    .into_iter()
                    .filter_map(|e| {
                        e.build(value_type).map(|ss| {
                            values.extend(ss.values);
                            ss.seg
                        })
                    })
                    .collect();
                let op = match op {
                    ConjOp::And => " and ",
                    ConjOp::Or => " or ",
                };

                seg.push_str(vs.join(op).as_str())
            }
            Wheres::In(key, fs) => {
                seg.push_str(key);
                seg.push_str(" in (");
                let vs = fs
                    .iter()
                    .map(|_| value_type.next())
                    .collect::<Vec<String>>();
                seg.push_str(vs.join(",").as_str());

                seg.push_str(")");
                values.extend(fs.into_iter())
            }
            Wheres::Not(fs) => {
                seg.push_str(" not ( ");
                if let Some(ss) = fs.build(value_type) {
                    seg.push_str(&ss.seg);
                    seg.push(')');

                    values.extend(ss.values);
                } else {
                    return None;
                }
            }
            Wheres::None => {
                return None;
            }
            Wheres::Compare(key, operator, value) => {
                seg.push_str(key);
                seg.push_str(" ");
                seg.push_str(operator);
                seg.push_str(" ");

                seg.push_str(&value_type.next());
                values.push(value);
            }
        }

        Some(SqlSeg { seg, values })
    }
}

pub struct SimpleUpdater<'a> {
    table: &'a str,
    setters: Vec<(&'a str, SqlValue<'a>)>,
    filters: Wheres<'a>,
}

impl<'a> SimpleUpdater<'a> {
    pub fn new(table: &'a str) -> Self {
        SimpleUpdater {
            table: &table,
            setters: vec![],
            filters: Wheres::and([]),
        }
    }

    pub fn set_if_some<T: Into<SqlValue<'a>>>(mut self, key: &'a str, value: Option<T>) -> Self {
        if let Some(v) = value {
            self.setters.push((key, v.into()));
        }

        self
    }

    pub fn set<T: Into<SqlValue<'a>>>(mut self, key: &'a str, v: T) -> Self {
        self.setters.push((key, v.into()));
        self
    }

    pub fn filters(mut self, filters: Wheres<'a>) -> Self {
        self.filters = filters;
        self
    }

    pub fn build(self, mut value_type: ValueType) -> Option<SqlSeg<'a>> {
        if self.setters.is_empty() {
            return None;
        }

        let mut sb = String::new();
        let mut values: Vec<SqlValue<'a>> = Vec::new();

        sb.push_str(" update ");
        sb.push_str(self.table);
        sb.push_str(" set ");

        let fields: Vec<String> = self
            .setters
            .into_iter()
            .map(|(key, v)| {
                values.push(v);
                format!(" {} = {} ", key, value_type.next())
            })
            .collect();
        sb.push_str(fields.join(", ").as_str());

        if let Some(filters) = self.filters.build(&mut value_type) {
            sb.push_str(" where ");
            sb.push_str(filters.seg.as_str());

            values.extend(filters.values);
        }

        Some(SqlSeg { seg: sb, values })
    }
}

pub enum SimpleQueryType<'a> {
    Where(Wheres<'a>),
    Comma(Vec<&'a str>),
    Raw(&'a str),
    Custom(Box<dyn QuerySeg<'a>>),
    Sub { alias: &'a str, query: SqlQuery<'a> },
}

pub struct SqlQuery<'a> {
    segs: Vec<SimpleQueryType<'a>>,
}

impl<'a> SqlQuery<'a> {
    pub fn new() -> Self {
        Self { segs: vec![] }
    }

    pub fn with_seg(mut self, seg: SimpleQueryType<'a>) -> Self {
        self.segs.push(seg);
        self
    }

    pub fn raw(mut self, seg: &'a str) -> Self {
        self.segs.push(SimpleQueryType::Raw(seg));
        self
    }

    pub fn wheres(mut self, filter: Wheres<'a>) -> Self {
        self.segs.push(SimpleQueryType::Where(filter));
        self
    }

    pub fn comma(mut self, filter: Vec<&'a str>) -> Self {
        self.segs.push(SimpleQueryType::Comma(filter));
        self
    }

    pub fn sub(mut self, alias: &'a str, query: SqlQuery<'a>) -> Self {
        self.segs.push(SimpleQueryType::Sub { alias, query });
        self
    }

    pub fn custom(mut self, custom: Box<dyn QuerySeg<'a>>) -> Self {
        self.segs.push(SimpleQueryType::Custom(custom));
        self
    }

    pub fn build(self, value_type: &mut ValueType) -> Option<SqlSeg<'a>> {
        if self.segs.is_empty() {
            return None;
        }

        let mut sb = String::new();
        let mut values: Vec<SqlValue<'a>> = Vec::new();

        for seg in self.segs {
            match seg {
                SimpleQueryType::Where(wr) => {
                    if let Some(ss) = wr.build(value_type) {
                        sb.push_str(" where ");
                        sb.push_str(&ss.seg);
                        values.extend(ss.values)
                    }
                }
                SimpleQueryType::Comma(vs) => {
                    sb.push_str(vs.join(", ").as_str());
                }
                SimpleQueryType::Raw(raw) => {
                    sb.push_str(&raw);
                }
                SimpleQueryType::Custom(custom) => {
                    if let Some(cs) = custom.build(value_type) {
                        sb.push_str(&cs.seg);
                        values.extend(cs.values)
                    }
                }
                SimpleQueryType::Sub { alias, query } => {
                    if let Some(s) = query.build(value_type) {
                        sb.push_str(" (");
                        sb.push_str(&s.seg);
                        sb.push_str(") ");
                        sb.push_str(&alias);
                        values.extend(s.values);
                    }
                }
            };
            if !sb.ends_with(" ") {
                sb.push(' ');
            }
        }

        Some(SqlSeg { seg: sb, values })
    }
}

pub trait QuerySeg<'a> {
    fn build(&self, value_type: &mut ValueType) -> Option<SqlSeg<'a>>;
}

pub struct LimitOffset {
    limit: u64,
    offset: Option<u64>,
}

impl LimitOffset {
    pub fn new(limit: u64) -> Self {
        Self {
            limit,
            offset: None,
        }
    }

    pub fn offset(mut self, offset: u64) -> Self {
        self.offset.replace(offset);

        self
    }

    pub fn offset_if_some(mut self, offset: Option<u64>) -> Self {
        self.offset = offset;

        self
    }

    pub fn to_box(self) -> Box<dyn QuerySeg<'static>> {
        Box::new(self)
    }
}

impl<'a> QuerySeg<'a> for LimitOffset {
    fn build(&self, _: &mut ValueType) -> Option<SqlSeg<'a>> {
        match self.offset {
            Some(v) => Some(SqlSeg {
                seg: format!("limit {} offset {}", self.limit, v),
                values: vec![],
            }),
            None => Some(SqlSeg {
                seg: format!("limit {}", self.limit),
                values: vec![],
            }),
        }
    }
}
