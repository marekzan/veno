use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VersionSchema {
    SemVer,
    DateVersioning,
    CompactDate,
    Serialized,
    BranchVersioning,
    Alphanumeric,
    Unknown,
}

impl VersionSchema {
    /// Returns the regex pattern associated with each enum variant
    fn regex(&self) -> &'static Regex {
        static SEMVER: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^\d+\.\d+\.\d+(-[a-zA-Z0-9]+(\.\d+)?)?$").unwrap());
        static DATE_VERSIONING: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^\d{4}\.\d{1,2}\.\d{1,2}(\.\d+)?$").unwrap());
        static COMPACT_DATE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{8}(\.\d+)?$").unwrap());
        static SERIALIZED: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d+$").unwrap());
        static BRANCH_VERSIONING: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^[a-zA-Z]+-\d+\.\d+\.\d+(-[a-zA-Z0-9]+)?$").unwrap());
        static ALPHANUMERIC: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^v?\d+\.\d+(-[a-zA-Z]+(\d+)?)?$").unwrap());

        match self {
            VersionSchema::SemVer => &SEMVER,
            VersionSchema::DateVersioning => &DATE_VERSIONING,
            VersionSchema::CompactDate => &COMPACT_DATE,
            VersionSchema::Serialized => &SERIALIZED,
            VersionSchema::BranchVersioning => &BRANCH_VERSIONING,
            VersionSchema::Alphanumeric => &ALPHANUMERIC,
            VersionSchema::Unknown => panic!("No regex for Unknown variant"),
        }
    }

    /// Attempts to match a string against all known patterns
    fn detect(input: &str) -> VersionSchema {
        [
            VersionSchema::SemVer,
            VersionSchema::DateVersioning,
            VersionSchema::CompactDate,
            VersionSchema::Serialized,
            VersionSchema::BranchVersioning,
            VersionSchema::Alphanumeric,
        ]
        .iter()
        .find(|schema| schema.regex().is_match(input))
        .cloned()
        .unwrap_or(VersionSchema::Unknown)
    }
}

// #[cfg(test)]
// pub mod test {
//
//     use super::*;
//
//     const SEM_VER: &str = "1.0.0";
//     const DATE_VERSIONING: &str = "2025.01.11";
//     const COMPACT_DATE: &str = "20250111.2";
//     const SERIALIZED: &str = "1";
//     const BRANCH_VERSIONING: &str = "main-1.0.3";
//     const ALPHANUMERIC: &str = "v1.2.0-beta1";
//     const UNKNOWN: &str = "invalid-version";
//
//     #[test]
//     fn test_version_schema() {
//         assert_eq!(VersionSchema::detect(SEM_VER), VersionSchema::SemVer);
//         assert_eq!(
//             VersionSchema::detect(DATE_VERSIONING),
//             VersionSchema::DateVersioning
//         );
//         assert_eq!(
//             VersionSchema::detect(COMPACT_DATE),
//             VersionSchema::CompactDate
//         );
//         assert_eq!(VersionSchema::detect(SERIALIZED), VersionSchema::Serialized);
//         assert_eq!(
//             VersionSchema::detect(BRANCH_VERSIONING),
//             VersionSchema::BranchVersioning
//         );
//         assert_eq!(
//             VersionSchema::detect(ALPHANUMERIC),
//             VersionSchema::Alphanumeric
//         );
//         assert_eq!(VersionSchema::detect(UNKNOWN), VersionSchema::Unknown);
//     }
// }
