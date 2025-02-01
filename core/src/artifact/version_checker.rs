use std::cmp::Ordering;

pub fn split_version(version: &str) -> Vec<&str> {
    let separators = ['.', '-', ':']; // Define your separators
    version.split(|c| separators.contains(&c)).collect()
}

// NOTE: this can be used for matching what versions to match. if i only want major and minor i
// could to smth like: y-y-n-n-n on a version like 1.2.0-stable-alpine
#[derive(Debug, PartialEq, Eq)]
pub enum VersionToken {
    Number,
    String,
}

pub fn infer_tokens(split_version: &[&str]) -> Vec<VersionToken> {
    split_version
        .iter()
        .map(|part| {
            if part.parse::<i32>().is_ok() {
                VersionToken::Number
            } else {
                VersionToken::String
            }
        })
        .collect()
}

pub fn find_newer_version(curr_v: &str, new_v: &str) -> Option<String> {
    let curr_v_split = split_version(curr_v);
    let curr_v_tokens = infer_tokens(&curr_v_split);
    let new_v_split = split_version(new_v);
    let new_v_tokens = infer_tokens(&new_v_split);

    // check if the version types are equal
    // [Number, Number, Number, String, String] ==
    // [Number, Number, Number, String, String]
    if curr_v_tokens == new_v_tokens {
        // we check if the merged version parts match their syntax
        if versions_compatible(&curr_v_split, &curr_v_tokens, &new_v_split)
            && version_is_newer(&curr_v_split, &curr_v_tokens, &new_v_split)
        {
            return Some(new_v.to_string());
        }
    }
    None
}

fn versions_compatible(
    curr_v_split: &[&str],
    curr_v_types: &[VersionToken],
    new_v_split: &[&str],
) -> bool {
    // we merge both splitted versions so we get a tuple to check
    // ["1", "0", "2", "alpine", "otel"] and
    // ["1", "3", "0", "alpine", "otel"] yields
    // [("1", "1"), ("0", "3"), ("2", "0"), ("alpine", "alpine"), ("otel", "otel")]
    let mut version_pairs = curr_v_split.iter().zip(new_v_split).enumerate();

    // closure which checks if the string values are equal -> we have a matching version structure
    // we just use true for number types since we need to check for them later
    let match_structure =
        |(index, (curr_v_value, new_v_value)): (usize, (&_, &_))| match curr_v_types[index] {
            VersionToken::Number => true,
            VersionToken::String => curr_v_value == new_v_value,
        };

    // we check if the version pairs match their structure
    version_pairs.all(match_structure)
}

fn version_is_newer(
    curr_v_split: &[&str],
    curr_v_tokens: &[VersionToken],
    new_v_split: &[&str],
) -> bool {
    for (i, (curr_part, new_part)) in curr_v_split.iter().zip(new_v_split.iter()).enumerate() {
        match curr_v_tokens[i] {
            VersionToken::Number => {
                let curr_num = curr_part.parse::<i32>().unwrap_or(0);
                let new_num = new_part.parse::<i32>().unwrap_or(0);
                match new_num.cmp(&curr_num) {
                    Ordering::Greater => return true,
                    Ordering::Less => return false,
                    Ordering::Equal => continue,
                }
            }
            VersionToken::String => {
                continue;
            }
        }
    }
    false
}
