#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;
use regex::Regex;
use crate::common::DesertMaps;

mod common;

fn main() {
    let input = common::read_lines("input.txt").expect("Unable to open input");
    let ans = process(input);
    println!("Part 1: {}", ans);
}

fn process(mut input: impl Iterator<Item = String>) -> usize {
    let map = DesertMaps::from_input(input);

    map.step_count(String::from("AAA"), &vec![String::from("ZZZ")])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_test() {
        let input = common::read_lines("sample.txt").expect("Unable to open sample text");
        assert_eq!(2, process(input));
    }

    #[test]
    fn part_one_sample_two_test() {
        let input = common::read_lines("sample_two.txt").expect("Unable to open sample text");
        assert_eq!(6, process(input));
    }
}