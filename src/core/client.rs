use crate::logger::init_logger;
use crate::utils::concatenate_url;
use log::debug;
use reqwest::Client;

pub struct JenkinsClient {
    client: Client,
    url: String,
    credentials: Option<Credentials>,
}

struct Credentials {
    username: Option<String>,
    password: Option<String>,
}

impl JenkinsClient {
    /// Create a new JenkinsClient without credentials
    pub fn new(url: String, verbose: bool) -> Self {
        init_logger(verbose);
        Self {
            client: Client::new(),
            url,
            credentials: None,
        }
    }

    /// Create a new JenkinsClient with credentials
    pub fn with_credentials(
        url: String,
        username: String,
        password: String,
        verbose: bool,
    ) -> Self {
        init_logger(verbose);
        Self {
            client: Client::new(),
            url,
            credentials: Some(Credentials {
                username: Some(username),
                password: Some(password),
            }),
        }
    }

    /// Perform a GET request to the given path and return the response as a string
    pub async fn get_path(&self, path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = concatenate_url(&self.url, path)?;
        debug!("GET {}", url);
        let mut request = self.client.get(&url);

        if let Some(credentials) = &self.credentials {
            request =
                request.basic_auth(credentials.get_username(), Some(credentials.get_password()));
        }

        let response = request.send().await?;
        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            Err(format!("Error: {}", response.status()).into())
        }
    }

    /// Perform a GET request to the given url and return the response as a
    /// string
    pub async fn get_url(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        debug!("GET {}", url);
        let mut request = self.client.get(url);

        if let Some(credentials) = &self.credentials {
            request =
                request.basic_auth(credentials.get_username(), Some(credentials.get_password()));
        }

        let response = request.send().await?;
        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            Err(format!("Error: {}", response.status()).into())
        }
    }

}

impl Credentials {
    /// Get username
    pub fn get_username(&self) -> &str {
        self.username.as_ref().map_or("", |s| s.as_str())
    }

    /// Get password
    pub fn get_password(&self) -> &str {
        self.password.as_ref().map_or("", |s| s.as_str())
    }
}
