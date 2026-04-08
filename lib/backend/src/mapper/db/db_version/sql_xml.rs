use chin_sql::DbType;
use quick_xml::Reader;
use quick_xml::events::Event;

#[derive(Debug, Clone)]
pub enum Fragment {
    Sql(String),
    Exec(String),
}

pub fn parse_sql_xml(input: &str, db_type: DbType) -> anyhow::Result<Vec<Fragment>> {
    let wrapped = format!("<root>{input}</root>");
    let mut reader = Reader::from_str(&wrapped);
    reader.config_mut().trim_text(false);

    let mut fragments: Vec<Fragment> = Vec::new();
    let mut current_sql = String::new();

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.local_name().as_ref() {
                b"match" => {
                    let resolved = resolve_match_block(&mut reader, db_type)?;
                    current_sql.push_str(&resolved);
                }
                b"dbtype" => {
                    let resolved = resolve_dbtype(&mut reader, db_type)?;
                    current_sql.push_str(&resolved);
                }
                b"execute" => {
                    let content = read_text_content(&mut reader)?;
                    flush_sql(&mut fragments, &mut current_sql);
                    fragments.push(Fragment::Exec(content.trim().to_string()));
                }
                b"root" => {}
                other => {
                    anyhow::bail!("unexpected element: {:?}", String::from_utf8_lossy(other))
                }
            },
            Event::Text(e) => {
                current_sql.push_str(&e.unescape()?);
            }
            Event::CData(e) => {
                current_sql.push_str(&String::from_utf8_lossy(&e.into_inner()));
            }
            Event::End(e) => {
                if e.local_name().as_ref() == b"root" {
                    flush_sql(&mut fragments, &mut current_sql);
                    break;
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(fragments)
}

fn flush_sql(fragments: &mut Vec<Fragment>, current_sql: &mut String) {
    let sql = std::mem::take(current_sql);
    for stmt in sql.split(";\n") {
        let stmt = stmt.trim();
        if !stmt.is_empty() {
            fragments.push(Fragment::Sql(stmt.to_string()));
        }
    }
}

fn resolve_dbtype(reader: &mut Reader<&[u8]>, db_type: DbType) -> anyhow::Result<String> {
    let type_name = read_text_content(reader)?;
    let type_name = type_name.trim().to_lowercase();

    let resolved = match type_name.as_str() {
        "int8" | "i64" => match db_type {
            DbType::Sqlite => "INTEGER",
            DbType::Postgres => "INT8",
        },
        "bool" => match db_type {
            DbType::Sqlite => "INTEGER",
            DbType::Postgres => "BOOL",
        },
        "int2" | "i16" => match db_type {
            DbType::Sqlite => "INTEGER",
            DbType::Postgres => "INT2",
        },
        "int4" | "i32" => match db_type {
            DbType::Sqlite => "INTEGER",
            DbType::Postgres => "INT4",
        },
        "float8" | "f64" => match db_type {
            DbType::Sqlite => "REAL",
            DbType::Postgres => "FLOAT8",
        },
        "text" => "TEXT",
        s if s.starts_with("varchar") => match db_type {
            DbType::Sqlite => "TEXT",
            DbType::Postgres => s,
        },
        _ => anyhow::bail!("unknown dbtype: {}", type_name),
    };

    Ok(resolved.to_string())
}

fn resolve_match_block(reader: &mut Reader<&[u8]>, db_type: DbType) -> anyhow::Result<String> {
    let db_key = match db_type {
        DbType::Sqlite => "sqlite",
        DbType::Postgres => "postgres",
    };

    let mut chosen: Option<String> = None;
    let mut fallback: Option<String> = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let tag = String::from_utf8_lossy(e.local_name().as_ref()).to_string();
                let content = read_text_content(reader)?;
                if tag == db_key {
                    chosen = Some(content);
                } else if tag == "default" {
                    fallback = Some(content);
                }
            }
            Event::End(e) => {
                if e.local_name().as_ref() == b"match" {
                    break;
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    chosen
        .or(fallback)
        .ok_or_else(|| anyhow::anyhow!("<match> block has no branch for {:?}", db_key))
}

fn read_text_content(reader: &mut Reader<&[u8]>) -> anyhow::Result<String> {
    let mut result = String::new();
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Text(e) => result.push_str(&e.unescape()?),
            Event::CData(e) => result.push_str(&String::from_utf8_lossy(&e.into_inner())),
            Event::End(_) => break,
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dbtype_and_match() {
        let input = r#"CREATE TABLE chnot_kind_rel (
  meta_otid <dbtype>int8</dbtype> NOT NULL,
  kind_id varchar(200) NOT NULL,
  tid <dbtype>int8</dbtype> NOT NULL
)"#;

        let sqlite_fragments = parse_sql_xml(input, DbType::Sqlite).unwrap();
        assert_eq!(sqlite_fragments.len(), 1);
        match &sqlite_fragments[0] {
            Fragment::Sql(sql) => {
                assert!(sql.contains("INTEGER"));
                assert!(!sql.contains("INT8"));
            }
            Fragment::Exec(_) => panic!("expected Sql"),
        }

        let postgres_fragments = parse_sql_xml(input, DbType::Postgres).unwrap();
        assert_eq!(postgres_fragments.len(), 1);
        match &postgres_fragments[0] {
            Fragment::Sql(sql) => {
                assert!(sql.contains("INT8"));
                assert!(!sql.contains("INTEGER"));
            }
            Fragment::Exec(_) => panic!("expected Sql"),
        }
    }

    #[test]
    fn test_match_block() {
        let input = r#"select
  <match><sqlite>1</sqlite><postgres>true</postgres></match>
from foo"#;

        let sqlite_fragments = parse_sql_xml(input, DbType::Sqlite).unwrap();
        match &sqlite_fragments[0] {
            Fragment::Sql(sql) => assert!(sql.contains("1")),
            Fragment::Exec(_) => panic!("expected Sql"),
        }

        let postgres_fragments = parse_sql_xml(input, DbType::Postgres).unwrap();
        match &postgres_fragments[0] {
            Fragment::Sql(sql) => assert!(sql.contains("true")),
            Fragment::Exec(_) => panic!("expected Sql"),
        }
    }

    #[test]
    fn test_execute_fragment() {
        let input = r#"<execute>v2_posthook</execute>"#;

        let fragments = parse_sql_xml(input, DbType::Sqlite).unwrap();
        assert_eq!(fragments.len(), 1);
        match &fragments[0] {
            Fragment::Exec(name) => assert_eq!(name, "v2_posthook"),
            Fragment::Sql(_) => panic!("expected Exec"),
        }
    }

    #[test]
    fn test_mixed_sql_and_execute() {
        let input = r#"CREATE TABLE foo (id <dbtype>int8</dbtype> NOT NULL);

<execute>some_function</execute>

insert into bar (col) select <match><sqlite>1</sqlite><postgres>true</postgres></match> from foo;"#;

        let fragments = parse_sql_xml(input, DbType::Postgres).unwrap();
        assert_eq!(fragments.len(), 3);
        match &fragments[0] {
            Fragment::Sql(sql) => {
                assert!(sql.contains("INT8"));
            }
            Fragment::Exec(_) => panic!("expected Sql"),
        }
        match &fragments[1] {
            Fragment::Exec(name) => assert_eq!(name, "some_function"),
            Fragment::Sql(_) => panic!("expected Exec"),
        }
        match &fragments[2] {
            Fragment::Sql(sql) => {
                assert!(sql.contains("true"));
            }
            Fragment::Exec(_) => panic!("expected Sql"),
        }
    }

    #[test]
    fn test_true_file() {
        let input = include_str!("../../../../../../data/sqls/v1-sql.xml");

        let fragments = parse_sql_xml(input, DbType::Postgres).unwrap();
    }
}
