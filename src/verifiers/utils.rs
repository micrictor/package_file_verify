use log::info;

use crate::result::VerificationResult;

/// Given a line or a list of lines containing rpm-format verification output, return the 
/// verification result.
pub fn get_verification_result_from_string(input: &str, file_path: &str) -> VerificationResult {
    // If no output is returned, we can assume that the result was successful
    if input.len() == 0 {
        info!("No output returned, all checks passed");
        return VerificationResult::from_string("........");
    }

    let mut verification_string: &str = "";
    for verification_line in input.lines() {
        if verification_line.ends_with(file_path) {
            (verification_string, _) = verification_line
                .rsplit_once(" ")
                .expect("failed to split line");
        }
    }

    println!("Verification string: {}", verification_string);

    VerificationResult::from_string(verification_string)
}