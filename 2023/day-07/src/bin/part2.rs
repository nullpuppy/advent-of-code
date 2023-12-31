#![allow(unused_imports)]
#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::ops::Add;
use utils::read_lines;
use day_07::{Card, Hand, HandType};

fn main() {
    let input = read_lines("input.txt").expect("Unable to open input");
    let ans = process(input);
    println!("Part 1: {}", ans);
}

fn process(input: impl Iterator<Item = String>) -> usize {
    let mut hands: Vec<_> = input.map(|input| Hand::new(input, true)).collect();

    hands.sort_by(Hand::compare);
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, hand)| acc + (rank + 1) * hand.bid())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_test() {
        let input = read_lines("sample.txt").expect("Unable to open sample text");
        assert_eq!(5905, process(input));
    }
}
