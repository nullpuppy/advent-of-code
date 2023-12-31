#![allow(unused_imports)]
#![allow(dead_code)]

use core::array::IntoIter;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Lines};
use utils::read_lines;
use day_05::Almanac;

fn main() {
    let mut input = read_lines("input.txt").unwrap();
    let ans = process(&mut input);
    println!("Part 1: {}", ans);
}

fn process(input: &mut impl Iterator<Item = String>) -> usize {
    let almanac = Almanac::from_input(input);
    let mut min_location = usize::MAX;

    for seed in almanac.seeds() {
        let loc = almanac.get_location_for_seed(*seed);
        if loc < min_location {
            min_location = loc;
        }
    }

    min_location
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_test() {
        let mut input = read_lines("sample_one.txt").unwrap();
        // let input = input.map(|line| line.unwrap()).collect();
        assert_eq!(35, process(&mut input));
    }
}
