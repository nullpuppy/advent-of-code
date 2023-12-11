#![allow(unused_imports)]
#![allow(dead_code)]
use utils::read_lines;
use day_09::OasisValue;

fn main() {
    let input = read_lines("input.txt").expect("Unable to open input");
    let ans = process(input);
    println!("Part 1: {}", ans);
}

fn process(input: impl Iterator<Item = String>) -> isize {
    let mut sum = 0;
    for line in input {
        let mut orig = OasisValue::from_string(&line, false);
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
        let input = read_lines("sample.txt").expect("Unable to open sample text");
        assert_eq!(114, process(input));
    }
}
