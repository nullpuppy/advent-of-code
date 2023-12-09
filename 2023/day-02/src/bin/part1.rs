use utils::read_lines;
mod common;
use common::GameRule;

fn main() {
    let input = read_lines("input.txt").unwrap();
    let rule = GameRule::new(12, 14, 13);
    let d1b_ans = process(rule, input);
    println!("Part 1: {}", d1b_ans);
}

fn process(rule: GameRule, input: impl Iterator<Item = String>) -> usize {
    let mut sum = 0;
    for line in input {
        let results = rule.process_game(line);
        if results.is_valid() {
            sum += results.id();
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_test() {
        let rule = GameRule::new(12, 14, 13);
        let input = read_lines("sample_one.txt").unwrap();
        assert_eq!(8, process(rule, input));
    }
}
