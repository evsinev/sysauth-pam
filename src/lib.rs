extern crate anyhow;
extern crate auto_args;
#[macro_use]
extern crate enum_display_derive;
extern crate env_logger;
extern crate gethostname;
extern crate log;
extern crate pam;
extern crate serde;
extern crate ureq;
extern crate humantime;
extern crate systemd_journal_logger;

mod sysauth_http_client;
mod sysauth_config;
mod pam_auth_action;
mod pam_env_mapper;
mod pam_result_code_mapper;
mod args_parser;
mod file_logger;
mod logger_init;
mod pam_debug;
mod pam_util;

use args_parser::{parse_args_pam, SysauthArgsSafe};
use log::{debug, error};
use logger_init::logger_init;
use pam::constants::PamResultCode::PAM_INCOMPLETE;
use pam::constants::{PamFlag, PamResultCode};
use pam::conv::Conv;
use pam::module::{PamHandle, PamHooks};
use pam_auth_action::PamAuthAction;
use pam_debug::debug_pam_arguments;
use pam_util::{pam_get_username, read_conv_text};
use std::ffi::CStr;

struct PamSysauth;
pam::pam_hooks!(PamSysauth);

impl PamHooks for PamSysauth {

    fn acct_mgmt(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        error!("account management");
        PamResultCode::PAM_SUCCESS
    }


    fn sm_authenticate(pamh: &mut PamHandle, pam_args: Vec<&CStr>, flags: PamFlag) -> PamResultCode {

        let safe_args: SysauthArgsSafe = parse_args_pam(&pam_args);
        logger_init(&safe_args);

        debug!("sm_authenticate with args {:?}", safe_args);
        debug_pam_arguments("sm_authenticate", &pam_args, flags, pamh);

        debug!("Getting username...");
        let _user = match pam_get_username(pamh) {
            Ok(user) => user,
            Err(err) => return err,
        };

        debug!("Getting conv...");
        let conv = match pamh.get_item::<Conv>() {
            Ok(Some(conv)) => conv,
            Ok(None) => {
                error!("No conv available");
                return PamResultCode::PAM_CONV_ERR;
            }
            Err(err) => {
                error!("Couldn't get pam_conv");
                return err;
            }
        };

        let password = match read_conv_text(&conv, safe_args.password_prompt.as_str()) {
            Ok(text) => text,
            Err(err) => return err,
        };

        let otp: String = if safe_args.otp {
            match read_conv_text(&conv, safe_args.otp_prompt.as_str()) {
                Ok(text) => text,
                Err(err) => return err,
            }
        } else {
            "".to_string()
        };

        let env = pam_env_mapper::pam_env_map(pamh);

        let user = match env.get("PAM_USER") {
            Some(user) => user,
            None => {
                error!("No PAM_USER in env");
                return PAM_INCOMPLETE;
            }
        };

        let pam_auth_action = PamAuthAction::new();
        let result = pam_auth_action.auth(user, &password, &otp, &env);
        result.unwrap_or_else(|err| {
            error!("Failed to authenticate: {:?}", err);
            PamResultCode::PAM_CRED_UNAVAIL
        })
    }

    fn sm_chauthtok(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        error!("sm_chauthtok");
        PamResultCode::PAM_SUCCESS
    }

    fn sm_close_session(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        error!("sm_close_session");
        PamResultCode::PAM_SUCCESS
    }

    fn sm_open_session(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        error!("sm_open_session");
        PamResultCode::PAM_SUCCESS
    }

    fn sm_setcred(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        error!("set credentials");
        PamResultCode::PAM_SUCCESS
    }

}

