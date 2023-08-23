use super::client::JenkinsClient;
use crate::logger::init_logger;
use crate::utils::search_substring;
use async_recursion::async_recursion;
use log::{warn, debug};

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
        let response = self.client.get_path("/api/json").await?;
        if search_substring(&response, r"Authentication required") {
            return Err("Authentication required".into());
        } else if search_substring(&response, r"Invalid password/token") {
            return Err("Invalid password/token".into());
        } else if search_substring(&response, r"missing the Overall/Read permission") {
            return Err("Missing the Overall/Read permission".into());
        } else {
            let response: serde_json::Value = serde_json::from_str(&response)?;

            // extract jobs from the parsed JSON and create a new JSON object
            let mut jobs_array = Vec::new();
            if let Some(jobs) = response.get("jobs").and_then(|jobs| jobs.as_array()) {
                debug!("Found {} jobs", jobs.len());
                debug!("Retrieving job info recursively");
                for job in jobs {
                    if let Some(job_url) = job.get("url").and_then(|url| url.as_str()) {
                        match self.get_jobs_recursive(job_url).await {
                            Ok(job_info) => jobs_array.push(job_info),
                            Err(e) => warn!("Error: {}", e),
                        }
                    }
                }
            }
            Ok(serde_json::Value::Array(jobs_array))
        }
    }

    /// Get job information recursively
    #[async_recursion]
    async fn get_jobs_recursive(
        &self,
        job_url: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        // Make a GET request to retrieve job information
        debug!("Retrieving job info from: {}", job_url);
        let response = self
            .client
            .get_url(format!("{}/api/json", job_url).as_str())
            .await?;

        // Parse the JSON response
        let json: serde_json::Value = serde_json::from_str(&response)?;

        // Create a JSON object to store job information
        let mut job_info = serde_json::json!({
            "name": json.get("name").and_then(|n| n.as_str()),
            "url": json.get("url").and_then(|url| url.as_str()),
        });

        // If the job has sub-jobs, recursively process them
        if let Some(sub_jobs) = json.get("jobs").and_then(|jobs| jobs.as_array()) {
            let mut sub_jobs_info = Vec::new();
            for sub_job in sub_jobs {
                if let Some(sub_job_url) = sub_job.get("url").and_then(|url| url.as_str()) {
                    let sub_job_info = self.get_jobs_recursive(sub_job_url).await?;
                    sub_jobs_info.push(sub_job_info);
                }
            }
            job_info["sub_jobs"] = serde_json::Value::Array(sub_jobs_info);
        }

        // If the job has builds, include their URLs
        if let Some(builds) = json.get("builds").and_then(|builds| builds.as_array()) {
            let build_urls: Vec<serde_json::Value> = builds
                .iter()
                .filter_map(|build| build.get("url").and_then(|url| url.as_str()))
                .map(|url| serde_json::Value::String(url.to_string()))
                .collect();
            job_info["builds"] = serde_json::Value::Array(build_urls);
        }

        Ok(job_info)
    }
}
