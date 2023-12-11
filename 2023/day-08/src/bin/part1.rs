#![allow(unused_imports)]
#![allow(dead_code)]

use regex::Regex;
use std::collections::HashMap;
use utils::read_lines;
use day_08::DesertMaps;

fn main() {
    let input = read_lines("input.txt").expect("Unable to open input");
    let ans = process(input);
    println!("Part 1: {}", ans);
}

fn process(input: impl Iterator<Item = String>) -> usize {
    let map = DesertMaps::from_input(input);

    map.step_count(String::from("AAA"), &[String::from("ZZZ")])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_test() {
        let input = read_lines("sample.txt").expect("Unable to open sample text");
        assert_eq!(2, process(input));
    }

    #[test]
    fn part_one_sample_two_test() {
        let input = read_lines("sample_two.txt").expect("Unable to open sample text");
        assert_eq!(6, process(input));
    }
}
