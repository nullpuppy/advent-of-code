/// Advent of Code 2024
///
/// Day 1, Part 1
/// Parse input for numbers, concat first and last of each line to make a 2-digit
/// number, and then sum all the numbers found
use utils::read_lines;

fn main() {
    let input = read_lines("input.txt").unwrap();
    let d1a_ans = process(input);
    println!("Part 1: {}", d1a_ans);
}

fn process(input: impl Iterator<Item = String>) -> u32 {
    let mut sum = 0;
    // for each line, find first and last digit
    for line in input {
        let mut digits: Vec<u32> = vec![];
        for c in line.chars() {
            if let Some(num) = c.to_digit(10) {
                digits.push(num);
            }
        }

        sum += digits[0] * 10 + digits.last().unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_test() {
        let input = read_lines("sample_one.txt").unwrap();
        assert_eq!(142, process(input));
    }
}
