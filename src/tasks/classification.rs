use std::collections::HashMap;

use super::Task;

pub struct FunCaptchaClassificationTask {
    image: String,
    question: String,
}

impl FunCaptchaClassificationTask {
    /// Creates a `FunCaptchaClassification` Task
    ///
    /// # Arguments
    ///
    /// * `encoded_image` - A string with the base64-encoded image to solve
    /// * `question` - The task question. Example: `Use the arrows to rotate the object to face in the direction of the hand`
    ///
    /// # Solution
    ///
    /// The solution to this task will be the tile index of the correct image as a string. **Remember: Tile indexes start at 0**
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustycap::tasks::classification::FunCaptchaClassificationTask;
    /// use std::fs::read_to_string;
    ///
    /// let image = read_to_string("image-base64.txt").expect("Unable to read image.");
    /// let task = FunCaptchaClassificationTask::new(image, "Use the arrows to rotate the object to face in the direction of the hand".to_string());
    /// ```
    pub fn new<T>(encoded_image: T, question: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            image: encoded_image.into(),
            question: question.into(),
        }
    }
}

impl Task for FunCaptchaClassificationTask {
    fn task_type(&self) -> &'static str {
        "FunCaptchaClassification"
    }

    fn properties(&self) -> HashMap<String, String> {
        HashMap::from([
            ("image".to_string(), self.image.clone()),
            ("question".to_string(), self.question.clone()),
        ])
    }
}
