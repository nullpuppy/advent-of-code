#![allow(unused_imports)]
#![allow(dead_code)]

use regex::Regex;
use std::collections::{HashMap, HashSet};
use utils::read_lines;
use day_08::DesertMaps;

fn main() {
    let input = read_lines("input.txt").expect("Unable to open input");
    let ans = process(input);
    println!("Part 1: {}", ans);
}

fn process(input: impl Iterator<Item = String>) -> usize {
    let map = DesertMaps::from_input(input);

    let steps: Vec<_> = map
        .start_nodes()
        .iter()
        .map(|node| map.step_count(node.to_owned(), map.end_nodes()))
        .collect();

    // Get prime factors for each number
    let factors: Vec<_> = steps.iter().map(|n| prime_factors(*n)).collect();

    // Find intersection of all prime factors. We should end up with at least
    // 1 value.
    let intersection = factors
        .iter()
        // each set of factors is currently a vector, convert to
        // HashSets to just find common factors, regardless of frequency
        .map(|f| HashSet::from_iter(f.iter()))
        .reduce(|l: HashSet<&usize>, r| {
            let intersection = l.intersection(&r);
            HashSet::from_iter(intersection.into_iter().copied())
        })
        .iter()
        // Map so reduce below is less annoying. There's gotta be a better way to do this though
        .flat_map(|v| v.to_owned())
        .collect::<Vec<_>>()
        .iter()
        .copied()
        .copied()
        // Find product of common factors, if no common factors, just return 1
        .reduce(|a, v| a * v)
        .unwrap_or(1);

    // Multiple non-prime factors together, and then multiply by shared prime factors once
    // at the end.
    // Somehow this works. Maths!
    factors
        .iter()
        .flatten()
        .fold(1, |acc, v| if *v == intersection { acc } else { acc * v })
        * intersection
}

fn prime_factors(mut num: usize) -> Vec<usize> {
    let mut factors = vec![];
    for i in 2..=(num / 2) {
        while num % i == 0 {
            num /= i;
            factors.push(i);
        }
    }
    if num > 1 {
        factors.push(num);
    }

    factors
}

fn get_steps(
    starting_node: String,
    maps: &HashMap<String, (String, String)>,
    pattern: &Vec<char>,
) -> usize {
    let mut step = 0;
    let mut count = 0;
    let mut current = starting_node;

    loop {
        current = match pattern[step] {
            'L' => maps[&current].0.to_owned(),
            'R' => maps[&current].1.to_owned(),
            _ => unreachable!(),
        };
        count += 1;
        if current.ends_with('Z') {
            break;
        }

        if step + 1 >= pattern.len() {
            step = 0;
        } else {
            step += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two_sample_test() {
        let input = read_lines("sample_three.txt").expect("Unable to open sample text");
        assert_eq!(6, process(input));
    }

    #[test]
    fn test_prime_factorization() {
        let expected = vec![2, 2, 2];
        assert_eq!(expected, prime_factors(8));

        let expected = vec![3, 5];
        assert_eq!(expected, prime_factors(15));

        let expected = vec![19];
        assert_eq!(expected, prime_factors(19));

        let expected = vec![2, 2, 2, 3];
        assert_eq!(expected, prime_factors(24));

        let expected = vec![2, 3, 3];
        assert_eq!(expected, prime_factors(18));

        let expected: Vec<usize> = vec![];
        assert_eq!(expected, prime_factors(0));
        assert_eq!(expected, prime_factors(1));
    }
}
