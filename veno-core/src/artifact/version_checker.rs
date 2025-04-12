use std::cmp::Ordering;

// NOTE: this can be used for matching what versions to match. if i only want major and minor i
// could to smth like: y-y-n-n-n on a version like 1.2.0-stable-alpine
#[derive(Debug, PartialEq, Eq)]
pub enum VersionPart<'a> {
    Number(i32),
    Text(&'a str),
}

fn parse_version(version: &str) -> Vec<VersionPart> {
    let separators = ['.', '-', ':'];

    version
        .split(|c| separators.contains(&c))
        .map(|part| match part.parse::<i32>() {
            Ok(num) => VersionPart::Number(num),
            Err(_) => VersionPart::Text(part),
        })
        .collect()
}

pub fn find_newer_version(curr_v: &str, new_v: &str) -> Option<String> {
    let current_version_parts = parse_version(curr_v);
    let new_version_parts = parse_version(new_v);

    if current_version_parts.len() != new_version_parts.len() {
        return None;
    }

    // setup an ordering to match the current and new version
    let mut overall_ordering = Ordering::Equal;

    // iterate over all parts as tuples
    // ["1", "0", "2", "alpine", "otel"] and
    // ["1", "3", "0", "alpine", "otel"] becomes
    // [("1", "1"), ("0", "3"), ("2", "0"), ("alpine", "alpine"), ("otel", "otel")]
    for (current_version_part, new_version_part) in
        current_version_parts.iter().zip(new_version_parts.iter())
    {
        match (current_version_part, new_version_part) {
            // if both parts are numbers we compare them and save the ordering
            (
                VersionPart::Number(current_version_number),
                VersionPart::Number(new_version_number),
            ) => {
                let current_ordering = new_version_number.cmp(current_version_number);
                // if both (current and overall) orderings are not equal, we found a difference
                if overall_ordering == Ordering::Equal && current_ordering != Ordering::Equal {
                    overall_ordering = current_ordering;
                }
            }
            // primarily incompatibility check
            (VersionPart::Text(curr_text), VersionPart::Text(new_text)) => {
                if curr_text != new_text {
                    return None; // incompatible version
                }
            }
            _ => {
                return None; // incompatible structure
            }
        }
    }

    // return only Some value if we found a compatible newer version
    if overall_ordering == Ordering::Greater {
        Some(new_v.to_string())
    } else {
        None // overall ordering was smaller or equal -> no new version
    }
}

#[cfg(test)]
mod tests {
    use super::find_newer_version;

    #[test]
    fn newer_versions_found() {
        assert_eq!(
            find_newer_version("1.2.3", "1.2.4"),
            Some("1.2.4".to_string()),
            "Patch increment"
        );
        assert_eq!(
            find_newer_version("1.2.8", "1.3.0"),
            Some("1.3.0".to_string()),
            "Minor increment"
        );
        assert_eq!(
            find_newer_version("1.9.9", "2.0.0"),
            Some("2.0.0".to_string()),
            "Major increment"
        );
        assert_eq!(
            find_newer_version("0.1.0", "0.10.0"),
            Some("0.10.0".to_string()),
            "Minor with two digits"
        );
        assert_eq!(
            find_newer_version("0.9.0", "0.10.0"),
            Some("0.10.0".to_string()),
            "Minor 9 to 10"
        );
        assert_eq!(
            find_newer_version("1", "2"),
            Some("2".to_string()),
            "Single digit major"
        );
        assert_eq!(
            find_newer_version("1.2.0-alpha", "1.2.1-alpha"),
            Some("1.2.1-alpha".to_string()),
            "Patch increment with same qualifier"
        );
        assert_eq!(
            find_newer_version("1.2.5-rc", "1.3.0-rc"),
            Some("1.3.0-rc".to_string()),
            "Minor increment with same qualifier"
        );
        assert_eq!(
            find_newer_version("1.8.0.RELEASE", "2.0.0.RELEASE"),
            Some("2.0.0.RELEASE".to_string()),
            "Major increment with same qualifier"
        );
        assert_eq!(
            find_newer_version("1.0.0-alpha.1", "1.0.0-alpha.2"),
            Some("1.0.0-alpha.2".to_string()),
            "Numeric increment after text part"
        );
        assert_eq!(
            find_newer_version("1.0.0-rc.9", "1.0.0-rc.10"),
            Some("1.0.0-rc.10".to_string()),
            "Numeric increment 9 to 10 after text part"
        );
        assert_eq!(
            find_newer_version("1:1.0.0", "1:1.1.0"),
            Some("1:1.1.0".to_string()),
            "Minor increment within epoch"
        );
        assert_eq!(
            find_newer_version("1:2.3.4", "2:0.0.0"),
            Some("2:0.0.0".to_string()),
            "Epoch increment"
        );
        assert_eq!(
            find_newer_version("1.02.3", "1.3.0"),
            Some("1.3.0".to_string()),
            "Leading zero in minor, new is newer"
        );
        assert_eq!(
            find_newer_version("1.2.03", "1.2.4"),
            Some("1.2.4".to_string()),
            "Leading zero in patch, new is newer"
        );

        assert_eq!(
            find_newer_version("v1.2.3", "v1.2.4"),
            Some("v1.2.4".to_string()),
            "'v' prefix, patch increment"
        );
        assert_eq!(
            find_newer_version("v1.9.0", "v1.10.0"),
            Some("v1.10.0".to_string()),
            "'v' prefix, minor increment 9 to 10"
        );
        assert_eq!(
            find_newer_version("v1", "v1.0"),
            None,
            "'v' prefix, different length"
        );
        assert_eq!(
            find_newer_version("2024", "2025"),
            Some("2025".to_string()),
            "Year increment"
        );
        assert_eq!(
            find_newer_version("2024.04", "2024.05"),
            Some("2024.05".to_string()),
            "Month increment"
        );
        assert_eq!(
            find_newer_version("2024.12", "2025.01"),
            Some("2025.01".to_string()),
            "Year increment takes precedence"
        );
        assert_eq!(
            find_newer_version("2024.1.9", "2024.1.10"),
            Some("2024.1.10".to_string()),
            "Day/Patch increment 9 to 10"
        );
    }

    #[test]
    fn older_versions_return_none() {
        assert_eq!(find_newer_version("1.2.4", "1.2.3"), None, "Patch older");
        assert_eq!(find_newer_version("1.3.0", "1.2.8"), None, "Minor older");
        assert_eq!(find_newer_version("2.0.0", "1.9.9"), None, "Major older");
        assert_eq!(find_newer_version("0.10.0", "0.9.0"), None, "Minor 10 to 9");
        assert_eq!(
            find_newer_version("2", "1"),
            None,
            "Single digit major older"
        );
        assert_eq!(
            find_newer_version("1.2.1-alpha", "1.2.0-alpha"),
            None,
            "Patch older with same qualifier"
        );
        assert_eq!(
            find_newer_version("1.3.0-rc", "1.2.5-rc"),
            None,
            "Minor older with same qualifier"
        );
        assert_eq!(
            find_newer_version("2.0.0.RELEASE", "1.8.0.RELEASE"),
            None,
            "Major older with same qualifier"
        );
        assert_eq!(
            find_newer_version("1.0.0-alpha.2", "1.0.0-alpha.1"),
            None,
            "Numeric older after text part"
        );
        assert_eq!(
            find_newer_version("1:1.1.0", "1:1.0.0"),
            None,
            "Minor older within epoch"
        );
        assert_eq!(
            find_newer_version("2:0.0.0", "1:2.3.4"),
            None,
            "Epoch older"
        );

        assert_eq!(
            find_newer_version("v1.2.4", "v1.2.3"),
            None,
            "'v' prefix, patch older"
        );
        assert_eq!(
            find_newer_version("v1.10.0", "v1.9.0"),
            None,
            "'v' prefix, minor older 10 to 9"
        );

        assert_eq!(find_newer_version("2025", "2024"), None, "Year older");
        assert_eq!(
            find_newer_version("2024.05", "2024.04"),
            None,
            "Month older"
        );
        assert_eq!(
            find_newer_version("2025.01", "2024.12"),
            None,
            "Year older takes precedence"
        );
        assert_eq!(
            find_newer_version("2024.1.10", "2024.1.9"),
            None,
            "Day/Patch older 10 to 9"
        );
    }

    #[test]
    fn equal_versions_return_none() {
        assert_eq!(find_newer_version("1.2.3", "1.2.3"), None, "Simple equal");
        assert_eq!(find_newer_version("2.0.0", "2.0.0"), None, "Major equal");
        assert_eq!(
            find_newer_version("1.2.3-alpha1", "1.2.3-alpha1"),
            None,
            "Equal with qualifier"
        );
        assert_eq!(
            find_newer_version("1:2.3.4", "1:2.3.4"),
            None,
            "Equal with epoch"
        );
        assert_eq!(
            find_newer_version("1.02.03", "1.2.3"),
            None,
            "Numerically equal despite leading zeros"
        );
        assert_eq!(
            find_newer_version("v1.0.0", "v1.0.0"),
            None,
            "'v' prefix, equal"
        );
        assert_eq!(
            find_newer_version("2024.04.11", "2024.04.11"),
            None,
            "CalVer equal"
        );
        assert_eq!(
            find_newer_version("v1", "v1"),
            None,
            "'v' prefix single part equal"
        );
        assert_eq!(find_newer_version("2025", "2025"), None, "Year equal");
    }

    #[test]
    fn incompatible_versions_return_none() {
        assert_eq!(
            find_newer_version("1.2.0-alpha", "1.2.0-beta"),
            None,
            "Different text qualifiers"
        );
        assert_eq!(
            find_newer_version("1.2.0.RELEASE", "1.2.0.SNAPSHOT"),
            None,
            "Different text qualifiers 2"
        );
        assert_eq!(
            find_newer_version("1.3.0-stable", "1.3.0-dev"),
            None,
            "Different text qualifiers 3"
        );
        assert_eq!(
            find_newer_version("1.0.0-rc1", "1.0.0-rc2"),
            None,
            "Text 'rc1' != 'rc2'"
        ); // Behandelt als Text
        assert_eq!(
            find_newer_version("1.2", "1.2.0"),
            None,
            "Different length (more parts)"
        );
        assert_eq!(
            find_newer_version("1.2.0", "1.2"),
            None,
            "Different length (fewer parts)"
        );
        assert_eq!(
            find_newer_version("1.0.0", "1.0.0-alpha"),
            None,
            "Different length (added qualifier)"
        );
        assert_eq!(
            find_newer_version("1.0.0-alpha", "1.0.0"),
            None,
            "Different length (removed qualifier)"
        );
        assert_eq!(find_newer_version("1.alpha", "1.0"), None, "Text vs Number");
        assert_eq!(find_newer_version("1.0", "1.alpha"), None, "Number vs Text");
        assert_eq!(
            find_newer_version("1.2.3", "1.2.alpha"),
            None,
            "Number vs Text later part"
        );
        assert_eq!(
            find_newer_version("1.2.3", "1.2.3+build1"),
            None,
            "Text '3' vs '3+build1'"
        );
        assert_eq!(
            find_newer_version("1.2.3+build1", "1.2.3+build2"),
            None,
            "Text '3+build1' vs '3+build2'"
        );
        assert_eq!(
            find_newer_version("1.2.3+build", "1.2.3"),
            None,
            "Text '3+build' vs '3'"
        );
        assert_eq!(
            find_newer_version("1..2", "1.0.2"),
            None,
            "Empty part vs number"
        );
        assert_eq!(
            find_newer_version("1.0.2", "1..2"),
            None,
            "Number vs empty part"
        );
        assert_eq!(
            find_newer_version("v1", "v2"),
            None,
            "'v' prefix, different text"
        );
        assert_eq!(
            find_newer_version("v1.0", "v2.0"),
            None,
            "'v' prefix, different text part"
        );
        assert_eq!(
            find_newer_version("v1", "1"),
            None,
            "'v' prefix vs number (incompatible types)"
        );
        assert_eq!(
            find_newer_version("1", "v1"),
            None,
            "Number vs 'v' prefix (incompatible types)"
        );
        assert_eq!(
            find_newer_version("v1.0", "1.0"),
            None,
            "'v' prefix vs number prefix (incompatible types)"
        );
        assert_eq!(
            find_newer_version("2024.04", "2024-04"),
            None,
            "CalVer number vs text"
        );
        assert_eq!(
            find_newer_version("2024.04", "2024.alpha"),
            None,
            "CalVer month vs text alpha"
        ); // Num(4) vs Text("alpha")
        assert_eq!(
            find_newer_version("2024.alpha", "2024.04"),
            None,
            "Text alpha vs CalVer month"
        );
    }
}
