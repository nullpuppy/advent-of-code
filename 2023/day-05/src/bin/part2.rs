#![allow(special_module_name)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use core::array::IntoIter;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Lines};
use utils::read_lines;

mod common;
use common::Almanac;

fn main() {
    let mut input = read_lines("input.txt").unwrap();
    let ans = process(&mut input);
    println!("Part 2: {}", ans);
}

fn process(input: &mut impl Iterator<Item = String>) -> usize {
    let almanac = Almanac::from_iter(input);

    for loc in 0..almanac.get_max_location() {
        let seed_from_loc = almanac.get_seed_for_location(loc);

        for chunk in almanac.seeds().chunks(2) {
            let start = chunk[0];
            let len = chunk[1];
            if seed_from_loc >= start && seed_from_loc < start + len {
                return loc;
            }
        }
    }

    usize::MAX
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_test() {
        let mut input = read_lines("sample_one.txt").unwrap();
        assert_eq!(46, process(&mut input));
    }
}
