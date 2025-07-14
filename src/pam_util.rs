use log::{debug, error};
use pam::constants::{PamResultCode, PAM_PROMPT_ECHO_OFF};
use pam::conv::Conv;
use pam::module::PamHandle;

pub fn read_conv_text(conv: &Conv, message: &str) -> Result<String, PamResultCode> {
    let text_option_cstr = conv.send(PAM_PROMPT_ECHO_OFF, message)?;

    let text_cstr = match text_option_cstr {
        Some(o) => o,
        None => return Err(PamResultCode::PAM_CRED_INSUFFICIENT),
    };

    let text: String = match text_cstr.to_str() {
        Ok(o) => o.to_string(),
        Err(error) => {
            error!("read_conv_text: Connect convert to utf-8 : {}", error);
            return Err(PamResultCode::PAM_CONV_ERR)
        },
    };

    debug!("read_conv_text: Read {} {:?} chars", message, text.len());

    Ok(text)
}

pub fn pam_get_username(pamh: &mut PamHandle) -> Result<String, PamResultCode> {
    let result = pamh.get_user(None);

    match result {
        Ok(o) => Ok(o),
        Err(err) => match err {
            PamResultCode::PAM_SUCCESS => {
                debug!("Username is empty. Will get username later");
                Ok(String::from(""))
            }
            _ => Err(err),
        },
    }
}
