//! The official CapBypass wrapper for Rust.
//!
//! # Example
//!
//! ```rust
//! use rustycap::Solver;
//! use rustycap::tasks::token::FunCaptchaTask;
//!
//! let solver = Solver::new("API-KEY");
//! let task = FunCaptchaTask::new("https://example.com/", "abcdefghijklmnop", "host:port");
//! let solution = solver.create_and_wait(task).await?;
//! ```

use std::time::Duration;

use anyhow::{anyhow, Result};
use models::{
    BalanceResponse, CreateTaskResponse, ServiceStatusResponse, TaskInfo, TaskResultResponse,
};
use reqwest::{get, Client};
use serde_json::json;
use tasks::Task;
use tokio::time::sleep;

#[cfg(feature = "image")]
pub mod image;
pub mod models;
pub mod tasks;

pub struct Solver {
    client_key: String,
    client: Client,
    delay: Duration,
    use_sleep: bool,
}

impl Solver {
    /// Creates a new solver instance
    ///
    /// # Arguments
    ///
    /// * `key` - A string containing the CapBypass key
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustycap::Solver;
    ///
    /// let solver = Solver::new("CB-XXXXXXXXX");
    /// ```
    pub fn new<T>(key: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            client_key: key.into(),
            client: Client::new(),
            use_sleep: true,
            delay: Duration::from_millis(2500),
        }
    }

    /// Configures the solver to not use a sleep in the `create_and_wait` function
    pub fn no_sleep(mut self) -> Self {
        self.use_sleep = false;

        self
    }

    /// Sets the delay for the `create_and_wait` function
    ///
    /// # Arguments
    ///
    /// * `delay` - The milliseconds to wait after each `get_task_info` request
    pub fn delay(mut self, delay: u64) -> Self {
        self.delay = Duration::from_millis(delay);

        self
    }

    /// Gets your current CapBypass balance
    pub async fn get_balance(&self) -> Result<BalanceResponse> {
        let data = json!({
            "clientKey": self.client_key,
        });
        let request = self
            .client
            .post("https://capbypass.com/api/getBalance")
            .header("content-type", "application/json")
            .json(&data)
            .send()
            .await?;
        let response = request.error_for_status()?;
        let response: BalanceResponse = response.json().await?;

        Ok(response)
    }

    /// Creates a task and returns the `task_id` and `error_id`
    ///
    /// # Arguments
    ///
    /// * `task` - A struct implementing the `Task` trait. Example: `FunCaptchaTask`
    pub async fn create_task<T>(&self, task: T) -> Result<CreateTaskResponse>
    where
        T: Task,
    {
        let task_data = task.serialize();
        let data = json!({
            "clientKey": self.client_key,
            "task": task_data,
        });
        let request = self
            .client
            .post("https://capbypass.com/api/createTask")
            .header("content-type", "application/json")
            .json(&data)
            .send()
            .await?;
        let response = request.error_for_status()?;
        let result: CreateTaskResponse = response.json().await?;

        Ok(result)
    }

    /// Retrieves the current task info for the given `task_id`
    pub async fn get_task_info<T>(&self, task_id: T) -> Result<TaskInfo>
    where
        T: Into<String>,
    {
        let task_id = task_id.into();
        let data = json!({
            "clientKey": self.client_key,
            "taskId": task_id,
        });
        let request = self
            .client
            .post("https://capbypass.com/api/getTaskResult")
            .header("content-type", "application/json")
            .json(&data)
            .send()
            .await?;
        let response = request.error_for_status()?;
        let response: TaskResultResponse = response.json().await?;

        match response.status.as_str() {
            "DONE" => {
                if let Some(solution) = response.solution {
                    Ok(TaskInfo::Done(solution))
                } else {
                    Ok(TaskInfo::Failed("Invalid response".to_string()))
                }
            }
            "FAILED" => Ok(TaskInfo::Failed(
                response
                    .error_description
                    .unwrap_or("errorDescription was null".to_string()),
            )),
            "DOES_NOT_EXIST" => Ok(TaskInfo::DoesNotExist),
            "PROCESSING" => Ok(TaskInfo::Processing),
            _ => Err(anyhow!(format!("Unknown task status: {}", response.status))),
        }
    }

    /// Creates a task and waits for the task to resolve
    /// #### ! This function loops and may use `tokio::time::sleep`
    ///
    /// # Arguments
    ///
    /// * `task` - A struct implementing the `Task` trait. Example: `FunCaptchaTask`
    pub async fn create_and_wait<T>(&self, task: T) -> Result<String>
    where
        T: Task,
    {
        let created = self.create_task(task).await?;

        if created.error_id == 0 {
            let task_id = created.task_id;

            loop {
                let status = self.get_task_info(&task_id).await?;

                match status {
                    TaskInfo::DoesNotExist => return Err(anyhow!("Task does not exist")),
                    TaskInfo::Done(solution) => return Ok(solution),
                    TaskInfo::Failed(reason) => {
                        return Err(anyhow!(format!("Task failed to solve: {}", reason)))
                    }
                    TaskInfo::Processing => {}
                };

                if self.use_sleep {
                    sleep(self.delay).await;
                }
            }
        } else {
            Err(anyhow!("Error"))
        }
    }
}

/// Fetches the current status and problems from the public api
pub async fn get_status() -> Result<ServiceStatusResponse> {
    let response = get("https://capbypass.com/api/status").await?;
    let data = response.json().await?;

    Ok(data)
}
