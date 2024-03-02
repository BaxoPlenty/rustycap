use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum TaskInfo {
    Processing,
    Failed,
    DoesNotExist,
    Done(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BalanceResponse {
    pub credits: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskResponse {
    pub error_id: usize,
    pub task_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskResultResponse {
    pub error_id: usize,
    pub status: String,
    pub solution: Option<String>,
}
