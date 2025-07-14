use log::{debug, error};
use pam::constants::PamResultCode;

pub fn pam_result_code_map(code: i32) -> PamResultCode {
    let result = match code {
          0 => PamResultCode::PAM_SUCCESS,
          1 => PamResultCode::PAM_OPEN_ERR,
          2 => PamResultCode::PAM_SYMBOL_ERR,
          3 => PamResultCode::PAM_SERVICE_ERR,
          4 => PamResultCode::PAM_SYSTEM_ERR,
          5 => PamResultCode::PAM_BUF_ERR,
          6 => PamResultCode::PAM_PERM_DENIED,
          7 => PamResultCode::PAM_AUTH_ERR,
          8 => PamResultCode::PAM_CRED_INSUFFICIENT,
          9 => PamResultCode::PAM_AUTHINFO_UNAVAIL,
         10 => PamResultCode::PAM_USER_UNKNOWN,
         11 => PamResultCode::PAM_MAXTRIES,
         12 => PamResultCode::PAM_NEW_AUTHTOK_REQD,
         13 => PamResultCode::PAM_ACCT_EXPIRED,
         14 => PamResultCode::PAM_SESSION_ERR,
         15 => PamResultCode::PAM_CRED_UNAVAIL,
         16 => PamResultCode::PAM_CRED_EXPIRED,
         17 => PamResultCode::PAM_CRED_ERR,
         18 => PamResultCode::PAM_NO_MODULE_DATA,
         19 => PamResultCode::PAM_CONV_ERR,
         20 => PamResultCode::PAM_AUTHTOK_ERR,
         21 => PamResultCode::PAM_AUTHTOK_RECOVERY_ERR,
         22 => PamResultCode::PAM_AUTHTOK_LOCK_BUSY,
         23 => PamResultCode::PAM_AUTHTOK_DISABLE_AGING,
         24 => PamResultCode::PAM_TRY_AGAIN,
         25 => PamResultCode::PAM_IGNORE,
         26 => PamResultCode::PAM_ABORT,
         27 => PamResultCode::PAM_AUTHTOK_EXPIRED,
         28 => PamResultCode::PAM_MODULE_UNKNOWN,
         29 => PamResultCode::PAM_BAD_ITEM,
         30 => PamResultCode::PAM_CONV_AGAIN ,
         31 => PamResultCode::PAM_INCOMPLETE ,
        _ => {
            error!("Unknown pam_result_code error: {:?}", code);
            PamResultCode::PAM_SYSTEM_ERR
        }
    };

    debug!("pam_result_code_mapping for {}: {:?}", code, result);

    result
}

