use serde::{Deserialize, Serialize};

pub enum TaskInfo {
    Processing,
    Failed,
    DoesNotExist,
    Done(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceResponse {
    pub credits: f32,
}