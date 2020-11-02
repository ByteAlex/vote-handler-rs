use crate::snowflake::Snowflake;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VoteRequest {
    pub bot: Snowflake,
    pub user: Snowflake,
    pub r#type: String,
    pub is_weekend: bool,
    pub query: Option<String>,
}