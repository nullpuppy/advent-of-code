#![allow(unused_imports)]
#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Add;

mod common;
use common::{Card, Hand, HandType};

fn main() {
    let input = common::read_lines("input.txt").expect("Unable to open input");
    let ans = process(input);
    println!("Part 1: {}", ans);
}

fn process(input: impl Iterator<Item = String>) -> usize {
    let mut hands: Vec<_> = input
        .map(|input| {
            Hand::new(input, false)
        }).collect();

    hands.sort_by(Hand::cmp);
    hands.iter()
        .enumerate()
        .fold(0, |acc, (rank, hand)| acc + (rank+1) * hand.bid())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_test() {
        let input = common::read_lines("sample.txt").expect("Unable to open sample text");
        assert_eq!(6440, process(input));
    }
}
