use anyhow::{anyhow, Context, Result};
use gethostname::gethostname;
use log::error;
use pam::constants::PamResultCode;
use pam_result_code_mapper::pam_result_code_map;
use std::collections::HashMap;
use std::path::Path;
use sysauth_http_client::{PamModuleType, PamRequest};

pub struct PamAuthAction {}

impl PamAuthAction {
    pub fn new() -> PamAuthAction {
        PamAuthAction {}
    }

    pub fn auth(&self, username: &str, password: &String, otp: &String, env: &HashMap<String, String>) -> Result<PamResultCode> {
        let config = crate::sysauth_config::SysAuthConfig::new();
        let pam_config = config.read_config(Path::new("/opt/sysauth-client/etc/sysauth-client.yaml"))?;
        let first_url = match pam_config.base_urls.first() {
            Some(url) => url,
            None => return Err(anyhow!("No base url at config")),
        };

        let client = crate::sysauth_http_client::SysAuthHttpClient::new(first_url);
        let pam_request = PamRequest {
            otp: Some(otp.to_string()),
            hostname: gethostname().into_string().unwrap(),
            std_in: password.into(),
            module_type: PamModuleType::AUTH,
            env: env.clone(),
        };

        let response = client.pam(&pam_request, &username.to_string()).context(format!("Could not authenticate {:?}", first_url))?;

        if let Some(message) = response.error_message {
            error!("Error message is {}", message)
        };

        Ok(pam_result_code_map(response.exit_code))
    }
}
