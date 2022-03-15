#[repr(u8)]
enum CheckFlags {
    Size = b'S',
    Mode = b'M',
    Checksum = b'5',
    MajorMinor = b'D',
    SymbolicLink = b'L',
    Owner = b'U',
    Group = b'G',
    ModificationTime = b'T',
}

impl Into<char> for CheckFlags {
    fn into(self) -> char {
        self as u8 as char
    }
}

fn get_character_for_result(result: CheckResult, flag_character: char) -> char {
    if result == CheckResult::Passed {
        return '.'
    }
    if result == CheckResult::Failed {
        return flag_character
    }

    return '?'
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CheckResult {
    // Represented by a ?
    Unsupported,
    // Represented by a .
    Passed,
    // Represented by that flag's character
    Failed,
}

#[derive(Debug)]
pub struct VerificationResult {
    pub(crate) size: CheckResult,
    pub(crate) mode: CheckResult,
    pub(crate) checksum: CheckResult,
    pub(crate) major_minor: CheckResult,
    pub(crate) symbolic_link: CheckResult,
    pub(crate) owner: CheckResult,
    pub(crate) group: CheckResult,
    pub(crate) modification_time: CheckResult,
    pub(crate) is_configuration: bool,
}

impl VerificationResult {
    // Construct a new `VerificationResult` instance with all checks unsupported
    pub fn unknown() -> Self {
        Self {
            size: CheckResult::Unsupported,
            mode: CheckResult::Unsupported,
            checksum: CheckResult::Unsupported,
            major_minor: CheckResult::Unsupported,
            symbolic_link: CheckResult::Unsupported,
            owner: CheckResult::Unsupported,
            group: CheckResult::Unsupported,
            modification_time: CheckResult::Unsupported,
            is_configuration: false,
        }
    }

    /// Parse an output string in RPM format
    /// # Examples
    /// 
    /// ```
    /// use package_file_verify::result::{CheckResult, VerificationResult};
    /// let result = VerificationResult::from_string(".?5????T c");
    /// assert_eq!(result.is_configuration(), true);
    /// assert_eq!(result.checksum(), CheckResult::Failed);
    /// assert_eq!(result.modification_time(), CheckResult::Failed);
    /// assert_eq!(result.size(), CheckResult::Passed);
    /// assert_eq!(result.mode(), CheckResult::Unsupported);
    /// ```
    pub fn from_string(input: &str) -> Self {
        let mut output_object = VerificationResult::unknown();
        for (pos, c) in input.char_indices() {
            let status: CheckResult = {
                match c {
                    '?' => CheckResult::Unsupported,
                    '.' => CheckResult::Passed,
                    _   => CheckResult::Failed,
                }
            };

            match pos {
                0 => output_object.size = status,
                1 => output_object.mode = status,
                2 => output_object.checksum = status,
                3 => output_object.major_minor = status,
                4 => output_object.symbolic_link = status,
                5 => output_object.owner = status,
                6 => output_object.group = status,
                7 => output_object.modification_time = status,
                _ => {},
            };
        }

        if input.len() > 8 && input.ends_with('c') {
            output_object.is_configuration = true;
        }
        output_object
    }

    /// Return the string representation of the check result
    ///
    /// # Examples
    /// 
    /// ```
    /// use package_file_verify::result::VerificationResult;
    /// let result = VerificationResult::unknown();
    /// let string = result.to_string();
    /// assert_eq!(string, "????????");
    /// ```
    /// 
    /// ```
    /// use package_file_verify::result::VerificationResult;
    /// let result = VerificationResult::from_string("??5????? c");
    /// let string = result.to_string();
    /// assert_eq!(string, "??5????? c");
    /// ```
    pub fn to_string(&self) -> String {
        let mut output_string: String = "".to_owned();
     
        output_string.push_str(
            &get_character_for_result(self.size, CheckFlags::Size.into()).to_string()
        );
        output_string.push_str(
            &get_character_for_result(self.mode, CheckFlags::Mode.into()).to_string()
        );
        output_string.push_str(
            &get_character_for_result(self.checksum, CheckFlags::Checksum.into()).to_string()
        );
        output_string.push_str(
            &get_character_for_result(self.major_minor, CheckFlags::MajorMinor.into()).to_string()
        );
        output_string.push_str(
            &get_character_for_result(self.symbolic_link, CheckFlags::SymbolicLink.into()).to_string()
        );
        output_string.push_str(
            &get_character_for_result(self.owner, CheckFlags::Owner.into()).to_string()
        );
        output_string.push_str(
            &get_character_for_result(self.group, CheckFlags::Group.into()).to_string()
        );
        output_string.push_str(
            &get_character_for_result(self.modification_time, CheckFlags::ModificationTime.into()).to_string()
        );
        
        if self.is_configuration {
            output_string.push_str(&" c");
        }

        return output_string
    }

    pub fn size(&self) -> CheckResult {
        self.size
    }

    pub fn mode(&self) -> CheckResult {
        self.mode
    }

    pub fn checksum(&self) -> CheckResult {
        self.checksum
    }

    pub fn major_minor(&self) -> CheckResult {
        self.major_minor
    }

    pub fn symbolic_link(&self) -> CheckResult {
        self.symbolic_link
    }

    pub fn owner(&self) -> CheckResult {
        self.owner
    }

    pub fn group(&self) -> CheckResult {
        self.group
    }

    pub fn modification_time(&self) -> CheckResult {
        self.modification_time
    }

    pub fn is_configuration(&self) -> bool {
        self.is_configuration
    }
}