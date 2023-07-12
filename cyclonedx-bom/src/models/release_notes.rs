use std::{convert::TryFrom, fmt::Display};

use chrono::{DateTime, FixedOffset};
use iri_string::types::IriString;

use crate::models::code::Issue;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ReleaseType {
    Major,
    #[default]
    Minor,
    Patch,
    PreRelease,
    Internal,
}

impl Display for ReleaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReleaseType::Major => write!(f, "major"),
            ReleaseType::Minor => write!(f, "minor"),
            ReleaseType::Patch => write!(f, "patch"),
            ReleaseType::PreRelease => write!(f, "pre-release"),
            ReleaseType::Internal => write!(f, "internal"),
        }
    }
}

impl TryFrom<&str> for ReleaseType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "major" => Ok(ReleaseType::Major),
            "minor" => Ok(ReleaseType::Minor),
            "patch" => Ok(ReleaseType::Patch),
            "pre-release" => Ok(ReleaseType::PreRelease),
            "internal" => Ok(ReleaseType::Internal),
            _ => Err("invalid release type"),
        }
    }
}

impl TryFrom<String> for ReleaseType {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        ReleaseType::try_from(value.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalizedString {
    pub locale: String,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ReleaseNotes {
    pub release_type: ReleaseType,
    pub title: Option<String>,
    pub featured_image: Option<IriString>,
    pub social_image: Option<IriString>,
    pub description: Option<String>,
    pub timestamp: Option<DateTime<FixedOffset>>,
    pub aliases: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub issues: Option<Vec<Issue>>,
    pub notes: Option<Vec<LocalizedString>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_datetimes_should_pass_validation() {
        let timestamp_result = DateTime::parse_from_rfc3339("1969-06-28T01:20:00.00-04:00");
        assert!(timestamp_result.is_ok());
    }
}
