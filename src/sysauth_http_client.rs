use anyhow::{anyhow, Context, Result};
use gethostname::gethostname;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::time::Duration;
use ureq::Agent;



pub(crate) struct SysAuthHttpClient {
    base_url: String,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Serialize, Deserialize, Display)]
pub enum PamModuleType {
    AUTH,
    ACCOUNT,
    PASSWORD,
    SESSION,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PamRequest {
    pub module_type: PamModuleType,
    pub hostname: String,
    pub env: HashMap<String, String>,
    pub std_in: String,
    pub otp: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PamResponse {
    pub exit_code: i32,
    pub error_message: Option<String>,
}


impl SysAuthHttpClient {
    pub(crate) fn new(base_url: &str) -> Self {
        SysAuthHttpClient {
            base_url: base_url.to_string(),
        }
    }

    pub fn pam(&self, request: &PamRequest, username: &String) -> Result<PamResponse> {
        let hostname = match gethostname().to_str() {
            Some(hostname) => hostname.to_string(),
            None => return Err(anyhow!("failed to convert to UTF-8")),
        };

        let agent: Agent = ureq::AgentBuilder::new()
            .timeout_read    ( Duration::from_secs(5) )
            .timeout_write   ( Duration::from_secs(5) )
            .timeout_connect ( Duration::from_secs(5) )
            .build();

        let module_type = &request.module_type.to_string().to_lowercase();
        let base_url = &self.base_url;

        let url = format!("{base_url}/pam/{module_type}/{hostname}/{username}");

        let request_json = serde_json::to_string(request)?;

        info!(">> {url} : {:?}", request.env);

        let http_response = agent.post(url.as_str())
            .timeout(Duration::new(5, 0))
            .set("Content-Type", "application/json")
            .send_string(request_json.as_str())
            .context("failed to send PAM request")?;


        let status_code = http_response.status();
        let response_body = http_response.into_string()?;

        info!("<< {status_code} : {response_body}");

        if status_code != 200 {
            error!("Response status is not 200 {:?} {:?}", status_code, response_body);
            return Err(anyhow!(status_code));
        }

        debug!("Received response: {}", response_body);

        let response: PamResponse = serde_json::from_str(&response_body).context("failed to parse response")?;

        Ok(response)
    }

}

#[cfg(test)]
mod tests {
    use logger_init::test_logger_try_init;
    use std::collections::HashMap;
    use sysauth_http_client::{PamModuleType, PamRequest, SysAuthHttpClient};

    #[test]
    fn test_pam_request() {
        test_logger_try_init();
        let client = SysAuthHttpClient::new("http://localhost:8090/sysauth");
        let response = client.pam(&PamRequest {
            otp: None,
            hostname: "test".to_string(),
            std_in: "password-1".to_string(),
            module_type: PamModuleType::AUTH,
            env: HashMap::from([("PAM_USER".to_string(), "test-3".to_string())]),
        }, &"username".to_string());
        println!("pam response is {:?}", response);
    }
}