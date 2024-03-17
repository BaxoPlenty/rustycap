use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum TaskInfo {
    Processing,
    Failed(String),
    DoesNotExist,
    Done(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TaskStatus {
    Unavailable,
    Instable,
    Operational,
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
    pub error_description: Option<String>,
    pub solution: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Problem {
    pub task: String,
    pub title: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServiceStatusResponse {
    pub tasks: HashMap<String, TaskStatus>,
    pub problems: Vec<Problem>,
}
