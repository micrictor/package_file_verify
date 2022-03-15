use log::debug;

use os_info;

use crate::result::VerificationResult;
use super::dpkg;

type Verifier = fn(file_path: &str) -> Option<VerificationResult>;

/// Get the verifier method for the current operating system
pub fn get_verifier() -> Option<Verifier> {
    let operating_system = os_info::get();

    get_verifier_method(operating_system.os_type())
}

/// Internal method for converting the os type to the appropriate verification
fn get_verifier_method(os_type: os_info::Type) -> Option<Verifier> {
    match os_type {
        os_info::Type::Debian
            | os_info::Type::Ubuntu
            | os_info::Type::Raspbian => {
                debug!("Using dpkg verifier for OS {}", os_type);
                Some(dpkg::verify)
            },
        _ => None,
    }
}