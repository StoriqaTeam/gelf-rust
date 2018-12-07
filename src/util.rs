use libc;
use regex::Regex;

/// Return the process-id (pid) of the current process
pub fn pid() -> i32 {
    unsafe { libc::getpid() }
}

pub fn hide_credentials<'a>(s: String) -> String {
    let mut res = s;
    let patterns = vec![
        (Regex::new("password\\s*:\\s*Some\\s*\\(\\s*\"[^\"]*\"\\s*\\)").unwrap(), "password: Some(\"****\")"),
        (Regex::new("(?P<n>\"[a-zA-Z0-9_ ]*[pP]assword[a-zA-Z0-9_ ]*\")\\s*:\\s*\"[^\"]*\"").unwrap(), "$n: \"****\"")
    ];

    for (pattern, replace) in patterns {
        res = pattern.replace_all(res.as_ref(), replace).to_string();
    }

    res
}