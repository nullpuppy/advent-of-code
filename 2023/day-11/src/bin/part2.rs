#![allow(unused_imports)]
#![allow(dead_code)]

use day_11::{age_galaxies, Galaxy, parse_input, sum_of_pairs};
use std::collections::HashSet;
use utils::read_lines;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
        let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();

    let input = read_lines("input.txt").expect("Unable to open input");
    let ans = process(input);
    println!("Part 1: {}", ans);
}

fn process(input: impl Iterator<Item = String>) -> usize {
    let (mut galaxies, density_horiz, density_vert) = parse_input(input);
    age_galaxies(&mut galaxies, &density_horiz, &density_vert, 1_000_000);
    sum_of_pairs(&galaxies)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two_sample_test() {
        let input = read_lines("sample.txt").expect("Unable to open sample text");
        assert_eq!(82000210, process(input));
    }

    fn part_two_sample_factor_10_test() {
        let input = read_lines("sample.txt").expect("Unable to open sample text");
        let (mut galaxies, density_horiz, density_vert) = parse_input(input);
        age_galaxies(&mut galaxies, &density_horiz, &density_vert, 10);
        assert_eq!(1030, sum_of_pairs(&galaxies));
    }

    fn part_two_sample_factor_100_test() {
        let input = read_lines("sample.txt").expect("Unable to open sample text");
        let (mut galaxies, density_horiz, density_vert) = parse_input(input);
        age_galaxies(&mut galaxies, &density_horiz, &density_vert, 100);
        assert_eq!(8410, sum_of_pairs(&galaxies));
    }

    fn part_two_sample_original_scaling_test() {
        let input = read_lines("sample.txt").expect("Unable to open sample text");
        let (mut galaxies, density_horiz, density_vert) = parse_input(input);
        age_galaxies(&mut galaxies, &density_horiz, &density_vert, 2);
        assert_eq!(374, sum_of_pairs(&galaxies));
    }
}
