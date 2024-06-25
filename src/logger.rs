use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceIdentifierRequest {
    pub id: i32,
}

pub const CONFIG_FILE_PATH: &str = "config.json";

pub mod logger {
    pub const DEFAULT_FORMAT: &str = "%a \"%r\" [%s] [%T] [%b] \"%{Referer}i\" \"%{User-Agent}i\" ";
    pub const DETAILED_FORMAT: &str =
        "%a \"%r\" [%s] [%D ms] [%b] \"%{Referer}i\" \"%{User-Agent}i\" ";
}
