use utils::read_lines;
mod common;
use common::GameRule;

fn main() {
    let input = read_lines("input.txt").unwrap();
    let d1b_ans = process(input);
    println!("Part 2: {}", d1b_ans);
}

fn process(input: impl Iterator<Item = String>) -> usize {
    let mut sum = 0;
    for line in input {
        let game_rule = process_game(line);
        sum += game_rule.power();
    }
    sum
}

fn process_game(game_str: String) -> GameRule {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    let mut iter = game_str.split_whitespace();
    iter.next(); // consume Game
    iter.next(); // consume game id

    while let Some(next) = iter.next() {
        if let Ok(num) = next.parse::<usize>() {
            let mut color = iter.next().unwrap().replace(",", "");
            if color.ends_with(";") {
                color = color.to_string().replace(";", "");
            }
            match color.as_str() {
                "red" => {
                    if num > red {
                        red = num;
                    }
                }
                "green" => {
                    if num > green {
                        green = num;
                    }
                }
                "blue" => {
                    if num > blue {
                        blue = num;
                    }
                }
                _ => {}
            };
        }
    }

    GameRule::new(red, green, blue)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_test() {
        let input = read_lines("sample_one.txt").unwrap();
        assert_eq!(2286, process(input));
    }
}
