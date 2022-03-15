use log::warn;

use std::process::Command;
use std::str;

use crate::result::VerificationResult;
use super::utils;

/// Verify a given file path using rpm search/verify
pub fn verify(file_path: &str) -> Option<VerificationResult> {
    let package_name: String = String::from_utf8(
            Command::new("/usr/bin/rpm")
                .arg("-qf")
                .arg(file_path)
                .output()
                .expect("failed to run rpm search")
                .stdout
        ).expect("failed to unwrap result str");
    if package_name.len() == 0 {
        warn!("No package found for file {}", file_path);
        return None
    }

    let raw_output = &Command::new("/usr/bin/rpm")
        .arg("-V")
        .arg(package_name)
        .output()
        .expect("failed to run rpm verify")
        .stdout;
    let verification_output: &str = str::from_utf8(raw_output)
        .expect("failed to unwrap result str");

    Some(utils::get_verification_result_from_string(verification_output, file_path))
}