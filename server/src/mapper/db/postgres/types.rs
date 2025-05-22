

#[macro_export]
macro_rules! to_sql {
    ($values:expr) => {
        $values
            .iter()
            .map(|e| {
                let v: &(dyn postgres_types::ToSql + Sync + Send) = e.into();
                v as &(dyn postgres_types::ToSql + Sync)
            })
            .collect::<Vec<&(dyn postgres_types::ToSql + Sync)>>()
            .as_slice()
    };
}

