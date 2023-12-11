use regex::Regex;
use std::collections::{HashMap, HashSet};
use utils::read_lines;

mod common;

use common::string_to_numset;

fn main() {
    let input = read_lines("input.txt").unwrap();
    let ans = process(input);
    println!("Part 2: {}", ans);
}

fn process(input: impl Iterator<Item = String>) -> usize {
    let mut card_copies: HashMap<usize, usize> = HashMap::new();

    let re =
        Regex::new(r"Card *(?<card_num>\d+): (?<winning>[\d ]+) \| (?<our_nums>[\d ]+)").unwrap();

    for line in input {
        let Some(captures) = re.captures(&line) else {
            panic!("No matches found for {line}");
        };

        let card_num = captures["card_num"].parse::<usize>().unwrap();
        let winning: HashSet<u32> = string_to_numset(String::from(&captures["winning"]));
        let our_nums: HashSet<u32> = string_to_numset(String::from(&captures["our_nums"]));

        let num_matching = winning.intersection(&our_nums).count();

        if let Some(idx) = card_copies.get_mut(&card_num) {
            *idx += 1;
        } else {
            card_copies.insert(card_num, 1);
        }

        if num_matching > 0 {
            // Previous "wins" will affect how many of the current card we should play.
            // for each copy, add the requisite number of subsequent cards.
            for _ in 0..card_copies[&card_num] {
                for j in card_num + 1..card_num + num_matching + 1 {
                    if let Some(idx) = card_copies.get_mut(&j) {
                        *idx += 1;
                    } else {
                        card_copies.insert(j, 1);
                    }
                }
            }
        }
    }

    card_copies
        .into_iter()
        .map(|(_, v)| v)
        .reduce(|l, r| l + r)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_test() {
        let input = read_lines("sample_one.txt").unwrap();
        assert_eq!(30, process(input));
    }
}
