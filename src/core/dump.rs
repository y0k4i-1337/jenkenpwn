use super::client::JenkinsClient;
use crate::logger::init_logger;
use crate::utils::search_substring;

pub struct Dumper {
    pub client: JenkinsClient,
}

// implement Dumper
impl Dumper {
    /// Create a new Dumper without credentials
    pub fn new(url: String, verbose: bool) -> Self {
        init_logger(verbose);
        Self {
            client: JenkinsClient::new(url, verbose),
        }
    }

    /// Create a new Dumper with credentials
    pub fn with_credentials(
        url: String,
        username: String,
        password: String,
        verbose: bool,
    ) -> Self {
        Self {
            client: JenkinsClient::with_credentials(url, username, password, verbose),
        }
    }

    /// Dump all jobs
    pub async fn dump_jobs(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let response = self.client.get("/api/json/").await?;
        if search_substring(&response, r"Authentication required") {
            return Err("Authentication required".into());
        } else if search_substring(&response, r"Invalid password/token") {
            return Err("Invalid password/token".into());
        } else if search_substring(&response, r"missing the Overall/Read permission") {
            return Err("Missing the Overall/Read permission".into());
        } else {
            let response: serde_json::Value = serde_json::from_str(&response)?;
            let jobs: serde_json::Value = response["jobs"].clone();
            Ok(jobs)
        }
    }
}
