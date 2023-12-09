#![allow(unused_imports)]
#![allow(dead_code)]

mod common;

use common::OasisValue;

fn main() {
    let input = common::read_lines("input.txt").expect("Unable to open input");
    let ans = process(input);
    println!("Part 2: {}", ans);
}

fn process(input: impl Iterator<Item = String>) -> isize {
    let mut sum = 0;
    for line in input {
        let mut orig = OasisValue::from_string(&line, true);
        orig.build_sequences();
        orig.extrapolate();

        sum += orig.get_next();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_test() {
        let input = common::read_lines("sample.txt").expect("Unable to open sample text");
        assert_eq!(2, process(input));
    }
}
