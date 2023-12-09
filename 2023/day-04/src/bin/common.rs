use std::collections::HashSet;

pub fn string_to_numset(s: String) -> HashSet<u32> {
    s.split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}