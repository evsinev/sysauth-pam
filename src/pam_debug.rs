use std::ffi::CStr;
use log::debug;
use pam::constants::{PamFlag, PAM_CHANGE_EXPIRED_AUTHTOK, PAM_DELETE_CRED, PAM_DISALLOW_NULL_AUTHTOK, PAM_ESTABLISH_CRED, PAM_REFRESH_CRED, PAM_REINITIALIZE_CRED, PAM_SILENT};
use pam::module::PamHandle;

pub fn debug_pam_arguments(name : &str, args: &Vec<&CStr>, flags: PamFlag, _pamh: &mut PamHandle) {
    debug!("call {name}: {:?}, {flags} flags: ", args);

    debug_pam_flag(flags, PAM_SILENT, "PAM_SILENT");
    debug_pam_flag(flags, PAM_DISALLOW_NULL_AUTHTOK, "PAM_DISALLOW_NULL_AUTHTOK");
    debug_pam_flag(flags, PAM_ESTABLISH_CRED, "PAM_ESTABLISH_CRED");
    debug_pam_flag(flags, PAM_DELETE_CRED, "PAM_DELETE_CRED");
    debug_pam_flag(flags, PAM_REINITIALIZE_CRED, "PAM_REINITIALIZE_CRED");
    debug_pam_flag(flags, PAM_REFRESH_CRED, "PAM_REFRESH_CRED");
    debug_pam_flag(flags, PAM_CHANGE_EXPIRED_AUTHTOK, "PAM_CHANGE_EXPIRED_AUTHTOK");
}

fn debug_pam_flag(flags: PamFlag, expected: PamFlag, name: &str) {
    if (flags & expected) == expected {
        debug!("Flag: {}, name: {}", flags, name);
    }
}
