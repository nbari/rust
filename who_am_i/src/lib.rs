use regex::Regex;
use std::{net::IpAddr, process::Command};

pub fn is_remote() -> bool {
    if let Ok(re) = Regex::new(r"\((.*)\)[\n\r]+$") {
        let output = Command::new("who")
            .arg("-T")
            .output()
            .expect("failed to execute process");
        if let Ok(raw) = String::from_utf8(output.stdout) {
            if let Some(caps) = re.captures(&raw) {
                if let Some(ip) = caps.get(1) {
                    // check ip
                    if let Ok(_) = ip.as_str().parse::<IpAddr>() {
                        return true;
                    }
                }
            }
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_remote(), false);
    }
}
