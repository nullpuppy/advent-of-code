#![allow(unused_imports)]
#![allow(dead_code)]
use utils::read_lines;

use day_10::{Coord2d, Direction, Grid, Tile};
use std::collections::{HashMap, HashSet, VecDeque};
use std::vec::Drain;

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
    let field = Grid::from_input(input);
    field.max_loop_distance()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_test() {
        let input = read_lines("sample.txt").expect("Unable to open sample text");
        assert_eq!(4, process(input));
    }

    #[test]
    fn part_one_sample_two_test() {
        let input = read_lines("sample_two.txt").expect("Unable to open sample text");
        assert_eq!(4, process(input));
    }

    #[test]
    fn part_one_sample_three_test() {
        let input = read_lines("sample_three.txt").expect("Unable to open sample text");
        assert_eq!(8, process(input));
    }
}
