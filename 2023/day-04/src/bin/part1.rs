#![allow(special_module_name)]

use regex::Regex;
use std::collections::HashSet;
use utils::read_lines;
use day_04::string_to_numset;

fn main() {
    let input = read_lines("input.txt").unwrap();
    let ans = process(input);
    println!("Part 1: {}", ans);
}

fn process(input: impl Iterator<Item = String>) -> usize {
    let mut sum = 0;

    let re =
        Regex::new(r"Card *(?<card_num>\d+): (?<winning>[\d ]+) \| (?<our_nums>[\d ]+)").unwrap();

    for line in input {
        let Some(captures) = re.captures(&line) else {
            panic!("No matches found for {line}");
        };

        let winning: HashSet<u32> = string_to_numset(String::from(&captures["winning"]));
        let our_nums: HashSet<u32> = string_to_numset(String::from(&captures["our_nums"]));

        let num_matching = winning.intersection(&our_nums).count() as u32;
        if num_matching > 0 {
            // Score starts at 1 for one match, then doubles for every match found after,
            // this amounts to 2^<num_matches-1>, 1 match would be 2^0, 3 matches would be
            // 2^2
            sum += 2usize.pow(num_matching - 1)
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_test() {
        let input = read_lines("sample_one.txt").unwrap();
        assert_eq!(13, process(input));
    }
}
