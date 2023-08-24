use super::client::JenkinsClient;
use crate::utils::{self, concatenate_url, extract_path, search_substring};
use crate::{logger::init_logger, utils::create_directory};
use async_recursion::async_recursion;
use log::{debug, info, warn};

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

    /// Create output directory for build based on base directory and build
    /// path (e.g. "job/MyJob/1")
    pub fn create_build_directory(
        &self,
        base_directory: &str,
        build_path: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let build_directory = format!("{}/{}", base_directory, build_path);
        create_directory(&build_directory)?;
        Ok(build_directory)
    }

    /// Dump all jobs
    pub async fn dump_jobs(
        &self,
        last_only: bool,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
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
                        match self.get_jobs_recursive(job_url, last_only).await {
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
        last_only: bool,
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
                    let sub_job_info = self.get_jobs_recursive(sub_job_url, last_only).await?;
                    sub_jobs_info.push(sub_job_info);
                }
            }
            job_info["sub_jobs"] = serde_json::Value::Array(sub_jobs_info);
        }

        // If the job has builds, include their URLs
        if let Some(builds) = json.get("builds").and_then(|builds| builds.as_array()) {
            if last_only {
                // get the first valid url among lastSuccessfulBuild.url, or
                // lastCompletedBuild.url, or lastStableBuild.url, or the first
                // element from "builds", and add it
                // to job_info as "builds" array
                let mut build_urls = Vec::new();
                if let Some(url) = json
                    .get("lastSuccessfulBuild")
                    .and_then(|build| build.get("url"))
                    .and_then(|url| url.as_str())
                {
                    build_urls.push(serde_json::Value::String(url.to_string()));
                } else if let Some(url) = json
                    .get("lastCompletedBuild")
                    .and_then(|build| build.get("url"))
                    .and_then(|url| url.as_str())
                {
                    build_urls.push(serde_json::Value::String(url.to_string()));
                } else if let Some(url) = json
                    .get("lastStableBuild")
                    .and_then(|build| build.get("url"))
                    .and_then(|url| url.as_str())
                {
                    build_urls.push(serde_json::Value::String(url.to_string()));
                } else if let Some(url) = builds
                    .first()
                    .and_then(|build| build.get("url"))
                    .and_then(|url| url.as_str())
                {
                    build_urls.push(serde_json::Value::String(url.to_string()));
                }
                job_info["builds"] = serde_json::Value::Array(build_urls);
            } else {
                // get all build urls and add them to job_info as "builds" array
                let build_urls: Vec<serde_json::Value> = builds
                    .iter()
                    .filter_map(|build| build.get("url").and_then(|url| url.as_str()))
                    .map(|url| serde_json::Value::String(url.to_string()))
                    .collect();
                job_info["builds"] = serde_json::Value::Array(build_urls);
            }
        }

        Ok(job_info)
    }

    /// Given a build url, dump consoleText and injectedEnvVars and save them
    /// in a directory based on the build path (e.g. "job/MyJob/1")
    pub async fn dump_build(
        &self,
        build_url: &str,
        output_directory: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Make a GET request to retrieve build information
        debug!("Retrieving build info from: {}", build_url);
        let response = self
            .client
            .get_url(format!("{}/api/json", build_url).as_str())
            .await
            .expect("Could not retrieve build info");

        // Parse the JSON response
        let build_info: serde_json::Value = serde_json::from_str(&response).expect("Invalid JSON");

        // Extract build path from url (e.g. "job/MyJob/1")
        let build_path = extract_path(build_url)?;
        info!("Dumping build: {}", build_path);

        let build_directory = self.create_build_directory(output_directory, &build_path)?;
        // Save build info to file
        let build_info_file = format!("{}/build_info.json", build_directory);
        debug!("Saving build info to {}", build_info_file);
        utils::save_json(&build_info, &build_info_file)?;

        // Get /consoleText for the build
        debug!("Retrieving consoleText for build {}", build_path);
        let console_text_url = concatenate_url(build_url, "/consoleText")?;
        let console_text = self
            .dump_console_text(&console_text_url)
            .await
            .unwrap_or_default();
        if !console_text.is_empty() {
            let console_text_file = format!("{}/consoleText", build_directory);
            debug!("Saving consoleText to {}", console_text_file);
            // async write to file
            tokio::fs::write(console_text_file, console_text).await?;
        } else {
            debug!("consoleText is empty");
        }

        // Get /injectedEnvVars for the build
        debug!("Retrieving injectedEnvVars for build {}", build_path);
        let injected_env_vars_url = concatenate_url(build_url, "/injectedEnvVars/api/json")?;
        let injected_env_vars = self
            .dump_injected_env_vars(&injected_env_vars_url)
            .await
            .unwrap_or_default();
        if !injected_env_vars.is_null() {
            let injected_env_vars_file = format!("{}/injectedEnvVars.json", build_directory);
            debug!("Saving injectedEnvVars to {}", injected_env_vars_file);
            utils::save_json(&injected_env_vars, &injected_env_vars_file)?;
        } else {
            debug!("injectedEnvVars is empty");
        }
        Ok(())
    }

    /// Dump builds for all jobs
    pub async fn dump_builds(
        &self,
        output_directory: &str,
        last_only: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // get all jobs
        let jobs = self.dump_jobs(last_only).await?;
        // if there are no jobs, return with an error
        if let Some(jobs) = jobs.as_array() {
            if jobs.is_empty() {
                return Err("No jobs found".into());
            }
        }
        // get all builds urls
        let mut builds_urls = Vec::new();
        self.get_builds_urls_recursive(&jobs, &mut builds_urls);
        // dump all builds
        for build_url in builds_urls {
            match self.dump_build(&build_url, output_directory).await {
                Ok(_) => {}
                Err(e) => warn!("Error dumping build {}: {}", build_url, e),
            }
        }
        Ok(())
    }

    /// Dump consoleText
    async fn dump_console_text(
        &self,
        console_text_url: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Make a GET request to retrieve consoleText
        debug!("Retrieving consoleText from: {}", console_text_url);
        let response = self.client.get_url(console_text_url).await?;

        Ok(response)
    }

    /// Dump injectedEnvVars
    async fn dump_injected_env_vars(
        &self,
        injected_env_vars_url: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        // Make a GET request to retrieve injectedEnvVars
        debug!("Retrieving injectedEnvVars from: {}", injected_env_vars_url);
        let response = self.client.get_url(injected_env_vars_url).await?;
        let json: serde_json::Value = serde_json::from_str(&response)?;

        Ok(json)
    }

    /// Iterate over a serde_json::Value recursively and get builds urls,
    /// returning a Vec<String> with all urls
    fn get_builds_urls_recursive(&self, json: &serde_json::Value, builds_urls: &mut Vec<String>) {
        if let Some(builds) = json.get("builds").and_then(|builds| builds.as_array()) {
            for build in builds {
                if let Some(url) = build.get("url").and_then(|url| url.as_str()) {
                    builds_urls.push(url.to_string());
                }
            }
        }
        if let Some(sub_jobs) = json.get("sub_jobs").and_then(|jobs| jobs.as_array()) {
            for sub_job in sub_jobs {
                self.get_builds_urls_recursive(sub_job, builds_urls);
            }
        }
    }
}
