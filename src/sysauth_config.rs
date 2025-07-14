use anyhow::{Context, Result};
use log::debug;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

pub struct SysAuthConfig {}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NssSocketAddress {
    pub from : String,
    pub to   : String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PamClientConfig {
    pub base_urls: Vec<String>,
    pub nss_socket_addresses: Vec<NssSocketAddress>,
}

impl SysAuthConfig {

    pub fn new() -> Self {
        SysAuthConfig {}
    }

    pub fn read_config(&self, filename: &Path) -> Result<PamClientConfig> {
        let file: File = File::open(filename).with_context(|| format!("Cannot open file {}", filename.to_str().unwrap()))?;
        let config: PamClientConfig = serde_yaml::from_reader(file).context("Cannot parse yaml file")?;
        debug!("Loaded config from {}", filename.to_str().unwrap());
        Ok(config)
    }

}

#[cfg(test)]
mod tests {
    use logger_init::test_logger_try_init;
    use std::path::Path;
    use sysauth_config::SysAuthConfig;

    #[test]
    fn test_read_config() {
        test_logger_try_init();
        let config : SysAuthConfig = SysAuthConfig::new();
        let pam_config = config.read_config(Path::new("./sysauth-client.yaml")).expect("Cannot read config");
        println!("{:#?}", pam_config);
    }
}