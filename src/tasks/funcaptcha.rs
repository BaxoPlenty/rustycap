use std::collections::HashMap;

use serde::Serialize;

use super::Task;

pub struct FunCaptchaTask {
    website_url: String,
    website_public_key: String,
    website_subdomain: Option<String>,
    data: Option<String>,
    proxy: String,
}

impl FunCaptchaTask {
    /// Creates a `FunCaptchaTask`
    ///
    /// # Arguments
    ///
    /// * `url` - A string containing the website url. Example: `https://example.com/`
    /// * `public_key` - A string containing the website public key
    /// * `proxy` - A string containing the proxy. Format: `host:port` OR `host:port:user:password`
    ///
    /// # Optional Data
    ///
    /// * `data(Serialize)` - This function can be used to pass data like a blob.
    /// * `subdomain(Into<String>)` - Sets an optional subdomain for funcaptcha. Example: `client-api.arkoselabs.com`
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustycap::tasks::funcaptcha::FunCaptchaTask;
    ///
    /// let task = FunCaptchaTask::new("https://example.com/", "abcdefghijklmnop", "host:port");
    /// ```
    pub fn new<T>(url: T, public_key: T, proxy: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            proxy: proxy.into(),
            website_public_key: public_key.into(),
            website_url: url.into(),
            website_subdomain: None,
            data: None,
        }
    }

    /// Sets the data content of the task
    ///
    /// # Arguments
    ///
    /// * `data` - A json-serializable struct
    pub fn data<T>(mut self, data: &T) -> Self
    where
        T: Serialize,
    {
        let json = serde_json::to_string(data);

        if let Ok(json) = json {
            self.data = Some(json);
        }

        self
    }

    /// Sets the special subdomain of arkoselabs.com
    ///
    /// # Arguments
    ///
    /// * `data` - A string containing the special subdomain
    pub fn subdomain<T>(mut self, data: T) -> Self
    where
        T: Into<String>,
    {
        self.website_subdomain = Some(data.into());

        self
    }
}

impl Task for FunCaptchaTask {
    fn task_type(&self) -> &'static str {
        "FunCaptchaTask"
    }

    fn properties(&self) -> HashMap<String, String> {
        let mut map = HashMap::from([
            (
                "websitePublicKey".to_string(),
                self.website_public_key.clone(),
            ),
            ("websiteURL".to_string(), self.website_url.clone()),
            ("proxy".to_string(), self.proxy.clone()),
        ]);

        if let Some(data) = &self.data {
            map.insert("data".to_string(), data.clone());
        }

        if let Some(subdomain) = &self.website_subdomain {
            map.insert("websiteSubdomain".to_string(), subdomain.clone());
        }

        map
    }
}
