use log::{debug, error};
use pam::items::{RHost, RUser, Service, Tty, User};
use pam::module::{PamHandle, PamResult};
use std::collections::HashMap;
use std::ffi::{CStr, OsString};
use std::os::unix::ffi::OsStringExt;

pub fn pam_env_map(pamh: &mut PamHandle) -> HashMap<String, String> {
    let mut map : HashMap<String, String> = HashMap::new();

    map.insert("PAM_TYPE".to_string(), "auth".to_string());

    // let service_result = pamh.get_item::<Service>();
    // let service_option = service_result.unwrap();
    // let service: Service = service_option.unwrap();
    // service.0;

    // standard items
    add_to_map_user   (&mut map, "PAM_USER"       , pamh.get_item::<User>());
    add_to_map_service(&mut map, "PAM_SERVICE"    , pamh.get_item::<Service>());
    add_to_map_tty    (&mut map, "PAM_TTY"        , pamh.get_item::<Tty>());
    add_to_map_rhost  (&mut map, "PAM_RHOST"      , pamh.get_item::<RHost>());
    add_to_map_ruser  (&mut map, "PAM_RUSER"      , pamh.get_item::<RUser>());

    // add_to_map(&mut map, "PAM_AUTHTOK"    , pamh.get_item::<AuthTok>());
    // add_to_map(&mut map, "PAM_OLDAUTHTOK" , pamh.get_item::<OldAuthTok>());
    // add_to_map(&mut map, "PAM_USER_PROMPT", pamh.get_item::<UserPrompt>());

    // environment
    for (key, value) in pam_getenvlist_wrapper(pamh) {
        if let Some(val) = value {
            map.insert(
                key.to_string_lossy().to_string()
                , val.to_string_lossy().to_string()
            );
        }
    }
    map
}

// fn add_to_map<T:Item>(map : &mut HashMap<String, String>, name: &str, result: PamResult<Option<T>>) {
//     match result {
//         Ok(Some(item)) => {
//             // debug!(" {name} = {}", value);
//             let raw : * const c_char = item.into_raw();
//             let text = unsafe { CStr::from_ptr(raw) }.to_string_lossy().into_owned();
//             map.insert(String::from(name), format!("{}", text));
//         }
//         Ok(None) => {
//             debug!(" {name} = no value");
//         }
//         Err(err) => {
//             debug!(" {name} = error = {:?}", err);
//         }
//     }
// }

fn add_to_map_service(map : &mut HashMap<String, String>, name: &str, result: PamResult<Option<Service>>) {
    match result {
        Ok(Some(item)) => {
            map.insert(String::from(name), item.0.to_string_lossy().to_string());
        }
        Ok(None) => {}
        Err(err) => {
            error!(" {name} = error = {:?}", err);
        }
    }
}

fn add_to_map_user(map : &mut HashMap<String, String>, name: &str, result: PamResult<Option<User>>) {
    match result {
        Ok(Some(item)) => {
            map.insert(String::from(name), item.0.to_string_lossy().to_string());
        }
        Ok(None) => {}
        Err(err) => {
            error!(" {name} = error = {:?}", err);
        }
    }
}

fn add_to_map_tty(map : &mut HashMap<String, String>, name: &str, result: PamResult<Option<Tty>>) {
    match result {
        Ok(Some(item)) => {
            map.insert(String::from(name), item.0.to_string_lossy().to_string());
        }
        Ok(None) => {}
        Err(err) => {
            error!(" {name} = error = {:?}", err);
        }
    }
}

fn add_to_map_rhost(map : &mut HashMap<String, String>, name: &str, result: PamResult<Option<RHost>>) {
    match result {
        Ok(Some(item)) => {
            map.insert(String::from(name), item.0.to_string_lossy().to_string());
        }
        Ok(None) => {}
        Err(err) => {
            error!(" {name} = error = {:?}", err);
        }
    }
}

fn add_to_map_ruser(map : &mut HashMap<String, String>, name: &str, result: PamResult<Option<RUser>>) {
    match result {
        Ok(Some(item)) => {
            map.insert(String::from(name), item.0.to_string_lossy().to_string());
        }
        Ok(None) => {}
        Err(err) => {
            error!(" {name} = error = {:?}", err);
        }
    }
}

/// pam_getenvlist function wrapper
/// Separate the env string to (key, value)
pub fn pam_getenvlist_wrapper(pamh: &mut PamHandle) -> Vec<(OsString, Option<OsString>)> {
    let mut result = Vec::new();
    let ptr = unsafe { pam_getenvlist(pamh) };
    debug!("pam_getenvlist_wrapper ptr = {:?}", ptr);
    let mut current = ptr;
    if !current.is_null() {
        while !(unsafe { *current }).is_null() {
            let one_line = unsafe { CStr::from_ptr(*current).to_bytes() };
            if !one_line.is_empty() {
                let mut pos = one_line.split(|x| *x == b'=');
                let key = OsString::from_vec(pos.next().unwrap().to_vec());
                let value = pos.next().map(|x| OsString::from_vec(x.to_vec()));
                // debug!("push {} {}", key.clone().to_str().unwrap(), value.clone().unwrap().to_str());
                result.push((key, value));
            }
            current = unsafe { current.add(1) };
        }
    }

    #[cfg(target_os = "linux")]
    unsafe { pam_misc_drop_env(ptr) };

    result
}

#[cfg(target_os = "linux")]
#[link(name = "pam_misc")]
extern "C" {
    pub fn pam_misc_drop_env(env: *mut *mut libc::c_char) -> *mut *mut libc::c_char;
}

extern "C" {
    pub fn pam_getenvlist(pamh: *mut PamHandle) -> *mut *mut libc::c_char;
}
