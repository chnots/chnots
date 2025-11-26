/// When there are some global data, use this.
/// e.g. in table KKV.
pub const NO_KSPACE: &str = "#NO_KSPACE#";
pub const CLIENT_ID_KEY: &str = "__CHNOT_CLINET_ID";
pub const ALL_ENDPOINTS_KEY: &str = "__CHNOT_ALL_ENDPOINTS";
pub const DB_VERSION_KEY: &str = "__CHNOT_DB_VERSION";

pub const DB_VERSION: &str = include_str!("../../../data/db.version");
