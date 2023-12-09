#![allow(unused_imports)]
#![allow(dead_code)]

mod common;

fn main() {
    let input = common::read_lines("input.txt").expect("Unable to open input");
    let ans = process(input);
    println!("Part 2: {}", ans);
}

fn process(mut input: impl Iterator<Item = String>) -> usize {

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_test() {
        let input = common::read_lines("sample_one.txt").expect("Unable to open sample text");
        assert_eq!(0, process(input));
    }
}
