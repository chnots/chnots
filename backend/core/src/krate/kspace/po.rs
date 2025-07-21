use chin_sql::{str_type::Varchar, time_type::TID, GenerateTableSchema};
use serde::{Deserialize, Serialize};


fn managers_to_sql(managers: Vec<String>) -> String {
    serde_json::to_string(&managers).unwrap()
}

#[derive(Debug, Clone, Deserialize, Serialize, GenerateTableSchema)]
pub struct KSpace {
    #[gts_primary]
    pub name: Varchar<500>,
    pub color: Varchar<100>,
    #[gts_type = "Text"]
    #[gts_tosql = "managers_to_sql"]
    pub managers: Vec<String>,
    #[gts_unique]
    #[gts_type = "i64"]    
    pub tid: TID,
}
