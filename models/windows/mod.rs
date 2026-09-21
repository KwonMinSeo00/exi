#[path = "10/rules.rs"]
pub mod win10;
#[path = "11/rules.rs"]
pub mod win11;
#[path = "7/rules.rs"]
pub mod win7;
#[path = "8/rules.rs"]
pub mod win8;

use super::{Rule, RuleFinding, evaluate_byte_rule};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowsProfile {
    Windows11,
}

pub fn rules(profile: WindowsProfile) -> &'static [Rule] {
    match profile {
        WindowsProfile::Windows11 => &win11::RULES,
    }
}

/*
    Return the profile's findings for a buffer already read by the caller.
    Passing these rules alone does not establish that the input is a valid PE.
*/
pub fn evaluate(profile: WindowsProfile, file_bytes: &[u8]) -> Vec<RuleFinding> {
    rules(profile)
        .iter()
        .map(|rule| evaluate_byte_rule(rule, file_bytes))
        .collect()
}
