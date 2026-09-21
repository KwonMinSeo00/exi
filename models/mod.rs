pub mod headers;
pub mod common;
pub mod windows;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rule {
    pub id: &'static str,
    pub offset: usize,
    pub expected: &'static [u8],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleStatus {
    Passed,
    ViolatesSpecification,
    Truncated,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleFinding {
    pub rule: Rule,
    pub status: RuleStatus,
    pub observed: Vec<u8>,
}

pub fn evaluate_byte_rule(rule: &Rule, file_bytes: &[u8]) -> RuleFinding {
    /*
        Slice before measuring the available bytes: adding offset and length
        could overflow for an invalid offset.
    */
    let remaining = file_bytes.get(rule.offset..);
    let available = remaining.unwrap_or_default();
    let observed = &available[..available.len().min(rule.expected.len())];

    let status = if remaining.is_none() || observed.len() < rule.expected.len() {
        RuleStatus::Truncated
    } else if observed == rule.expected {
        RuleStatus::Passed
    } else {
        RuleStatus::ViolatesSpecification
    };

    RuleFinding {
        rule: *rule,
        status,
        observed: observed.to_vec(),
    }
}
