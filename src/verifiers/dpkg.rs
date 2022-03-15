use log::warn;

use std::process::Command;
use std::str;

use crate::result::VerificationResult;
use super::utils;

/// Verify a given file path using the dkpg search/verify
/// 
/// # Examples
/// 
/// ```
/// use package_file_verify::verifiers::dpkg;
/// use package_file_verify::result::CheckResult;
/// 
/// let result = dpkg::verify("/bin/bash")
///     .expect("failed");
/// assert_eq!(result.checksum(), CheckResult::Passed);
/// assert_eq!(result.is_configuration(), false);
/// ```
pub fn verify(file_path: &str) -> Option<VerificationResult> {
    let search_output: String = String::from_utf8(
            Command::new("/usr/bin/dpkg")
                .arg("-S")
                .arg(file_path)
                .output()
                .expect("failed to run dpkg search")
                .stdout
        ).expect("failed to unwrap result str");
    if search_output.len() == 0 {
        warn!("No package found for file {}", file_path);
        return None
    }

    let (package_name, _) = search_output.split_once(":")
        .expect("failed to split search output");

    let raw_output = &Command::new("/usr/bin/dpkg")
        .arg("-V")
        .arg("--verify-format=rpm")
        .arg(package_name)
        .output()
        .expect("failed to run dpkg verify")
        .stdout;
    let verification_output: &str = str::from_utf8(raw_output)
        .expect("failed to unwrap result str");

    Some(utils::get_verification_result_from_string(verification_output, file_path))
}