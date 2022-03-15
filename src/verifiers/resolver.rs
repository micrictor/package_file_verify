use log::{debug, warn};

use os_info;
use os_info::Type;

use crate::result::VerificationResult;
use super::dpkg;
use super::rpm;

type Verifier = fn(file_path: &str) -> Option<VerificationResult>;

/// Get the verifier method for the current operating system
pub fn get_verifier() -> Option<Verifier> {
    let operating_system = os_info::get();

    get_verifier_method(operating_system.os_type())
}

/// Internal method for converting the os type to the appropriate verification
fn get_verifier_method(os_type: Type) -> Option<Verifier> {
    match os_type {
        Type::Debian
        | Type::Ubuntu
        | Type::Raspbian => {
            debug!("Using dpkg verifier for OS {}", os_type);
            Some(dpkg::verify)
        },
        Type::Redhat
        | Type::RedHatEnterprise
        | Type::Amazon
        | Type::CentOS
        | Type::Fedora
        | Type::OracleLinux => {
            debug!("Using rpm verifier for OS {}", os_type);
            Some(rpm::verify)
        }
        _ => {
            warn!("No verifier found for OS {}", os_type);
            None
        },
    }
}