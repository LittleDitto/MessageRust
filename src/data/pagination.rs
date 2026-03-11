use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination{
    pub page: Option<i64>,
    pub limit: Option<i64>,
}