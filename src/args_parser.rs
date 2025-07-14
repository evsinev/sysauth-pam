use auto_args::AutoArgs;
use log::LevelFilter;
use std::ffi::CStr;

#[derive(Debug, AutoArgs, PartialEq)]
pub struct SysauthArgs {
    otp: Option<bool>,
    otp_prompt: Option<String>,
    password_prompt: Option<String>,
    logger_type: Option<LoggerKind>,
    env_log_filter: Option<String>,
    file_log_path: Option<String>,
    file_log_level: Option<String>,
    journald_log_level: Option<String>,
}

#[derive(Debug, PartialEq, AutoArgs)]
pub enum LoggerKind {
    Env,
    File,
    Journald,
}

#[derive(Debug)]
pub struct SysauthArgsSafe {
    pub otp: bool,
    pub otp_prompt: String,
    pub password_prompt: String,
    pub logger_type: LoggerKind,
    pub env_log_filter: String,
    pub file_log_path: String,
    pub file_log_level: LevelFilter,
    pub journald_log_level: LevelFilter,
}

pub fn parse_args_pam(pam_args: &Vec<&CStr>) -> SysauthArgsSafe {
    let args: Vec<String> = pam_args
        .iter()
        .map(|x| x.to_string_lossy().into_owned())
        .collect();

    parse_args(args)
}
fn parse_args(args: Vec<String>) -> SysauthArgsSafe {
    let mut with_program_name : Vec<String> = args;
    with_program_name.insert(0, "program_name".to_string());

    let default = SysauthArgsSafe {
        otp             : false,
        otp_prompt      : "OTP: ".to_string(),
        password_prompt : "Password: ".to_string(),
        logger_type     : LoggerKind::Journald,
        env_log_filter  : "info".to_string(),
        file_log_path   : "/tmp/pam_sysauth.log".to_string(),
        file_log_level  : LevelFilter::Debug,
        journald_log_level : LevelFilter::Info,
    };

    match SysauthArgs::from_iter(with_program_name) {
        Ok(parsed) => {
            SysauthArgsSafe {
                otp                : parsed.otp.unwrap_or(default.otp),
                otp_prompt         : parsed.otp_prompt.unwrap_or(default.otp_prompt),
                password_prompt    : parsed.password_prompt.unwrap_or(default.password_prompt),
                logger_type        : parsed.logger_type.unwrap_or(default.logger_type),
                env_log_filter     : parsed.env_log_filter.unwrap_or(default.env_log_filter),
                file_log_path      : parsed.file_log_path.unwrap_or(default.file_log_path),
                file_log_level     : parse_log_level(parsed.file_log_level, default.file_log_level),
                journald_log_level : parse_log_level(parsed.journald_log_level, default.journald_log_level),
            }
        }
        Err(err) => {
            println!(
                "Cannot parse arguments: {}. return default {:?}",
                err, default
            );
            println!("Help {}", SysauthArgs::help());
            default
        }
    }
}

fn parse_log_level(input: Option<String>, default_level: LevelFilter) -> LevelFilter {
    match input {
        Some(level) => match level.to_uppercase().as_str() {
            "OFF"   => LevelFilter::Off,
            "ERROR" => LevelFilter::Error,
            "WARN"  => LevelFilter::Warn,
            "INFO"  => LevelFilter::Info,
            "DEBUG" => LevelFilter::Debug,
            "TRACE" => LevelFilter::Trace,
            _ => default_level,
        }
        None => default_level,
    }
}

#[cfg(test)]
mod tests {
    use args_parser::{parse_args, LoggerKind, SysauthArgs};
    use auto_args::AutoArgs;
    use log::LevelFilter;

    #[test]
    fn test_parse_args() {
        let args = parse_args(vec![
            "--otp".to_string()
            , "--otp-prompt".to_string()
            , "Hello From OTP :".to_string()
            , "--logger-type-file".to_string()
            , "--file-log-level".to_string()
            , "WARN".to_string()
        ]);
        println!("args {:?}", args);

        println!("{}", SysauthArgs::help());

        assert_eq!(args.otp, true);
        assert_eq!(args.otp_prompt, "Hello From OTP :".to_string());
        assert_eq!(args.logger_type, LoggerKind::File);
        assert_eq!(args.file_log_level, LevelFilter::Warn);
    }
}
