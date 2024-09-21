#[macro_export]
macro_rules! to_sql {
    ($values:expr) => {
        $values
            .iter()
            .map(|e| {
                let v: &(dyn ToSql + Sync + Send) = e.into();
                v as &(dyn ToSql + Sync)
            })
            .collect::<Vec<&(dyn ToSql + Sync)>>()
            .as_slice()
    };
}
