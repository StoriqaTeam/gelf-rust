use libc;
use regex::Regex;

/// Return the process-id (pid) of the current process
pub fn pid() -> i32 {
    unsafe { libc::getpid() }
}

pub fn hide_passwords(s: String) -> String {
    let re = Regex::new("password\\s*:\\s*Some\\(\".*\"\\)").unwrap();
    re.replace(s, "password: Some(\"****\")")
}