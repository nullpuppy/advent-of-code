#![allow(unused_imports)]
#![allow(dead_code)]
use utils::read_lines;

mod common;
use common::parse_line;

fn main() {
    let input = read_lines("input.txt").expect("Unable to open input");
    let ans = process(input);
    println!("Part 1: {}", ans);
}

fn process(mut input: impl Iterator<Item = String>) -> usize {
    let times: Vec<_> = parse_line(input.next().unwrap());
    let distances: Vec<_> = parse_line(input.next().unwrap());

    let mut result = 1;

    for race in 0..times.len() {
        let race_duration = times[race];
        let winning_distance = distances[race];

        let mut margin = 0;

        // We don't care about the case where the button isn't pressed at all, or
        // when it is pressed for the entire duration, as that leads to no distance traveled.
        for wait in 1..race_duration {
            // duration of wait == speed.
            // speed * duration remaining == distance.
            if wait * (race_duration-wait) > winning_distance {
                margin += 1;
            }
        }

        println!("{race_duration} {winning_distance} {margin}");

        result *= margin;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_test() {
        let input = read_lines("sample.txt").expect("Unable to open sample text");
        assert_eq!(288, process(input));
    }
}
