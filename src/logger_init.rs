use args_parser::{LoggerKind, SysauthArgsSafe};
use env_logger::Env;
use systemd_journal_logger::JournalLog;
use file_logger::{FileLogger};

pub fn logger_init(safe_args: &SysauthArgsSafe) {
    match safe_args.logger_type {
        LoggerKind::Env => {
            init_env_logger(safe_args);
        }
        LoggerKind::File => {
            FileLogger::try_init(safe_args.file_log_path.clone(), safe_args.file_log_level);
        }
        LoggerKind::Journald => {
            init_journald_logger(safe_args);
        }
    }
}

fn init_env_logger(safe_args: &SysauthArgsSafe) {
    let env = Env::default()
        .default_filter_or(safe_args.env_log_filter.clone());

    let _logger = env_logger::Builder::from_env(env)
        .try_init();
}

fn init_journald_logger(safe_args: &SysauthArgsSafe) {
    match JournalLog::new() {
        Ok(journald) => {
            let result = journald
                .with_syslog_identifier("sysauth-pam".to_string())
                .install();
            if let Err(err) = result {
                init_env_logger(safe_args);
                log::error!("Cannot install Journald Logger: {}", err);
            }
        }
        Err(err) => {
            init_env_logger(safe_args);
            log::error!("Cannot create Journald Logger: {}", err);
        }
    }
    log::set_max_level(safe_args.journald_log_level);
}

#[allow(dead_code)]
pub fn test_logger_try_init() {
    let _ = env_logger::builder().is_test(true).try_init();
}