/// Advent of Code 2023
///
/// Day 1, Part 2
/// Expanding on part one, there are words that equate to numbers. Parse these as digits,
/// and then basically do the same as in part 1.
///
/// Number-words can overlap (i.e., oneight is 1 and 8)
use utils::read_lines;

fn main() {
    let input = read_lines("input.txt").unwrap();
    let d1b_ans = process(input);
    println!("Part 2: {}", d1b_ans);
}

fn process(input: impl Iterator<Item = String>) -> u32 {
    let mut sum = 0;

    let number_words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    // for each line, find first and last digit
    for line in input {
        let mut digits: Vec<u32> = vec![];
        let mut word = String::new();
        for c in line.chars() {
            if let Some(num) = c.to_digit(10) {
                digits.push(num);
                word = String::new();
            } else {
                // If we couldn't parse a number, let's see if we have a possible word for a number
                word.push(c);
                if let Some(v) = number_words.iter().position(|w| word.eq(w)) {
                    // println!("Found word number: {} pushing {}", word, v+1);
                    digits.push(v as u32 + 1);
                    word = String::from(c);
                } else {
                    loop {
                        if word.len() == 0 {
                            break;
                        }
                        if !number_words.iter().any(|w| w.starts_with(word.as_str())) {
                            // println!("{} is not a probable number, resetting", word);
                            let mut tmp = word.chars();
                            tmp.next();
                            let count = tmp.clone().count();

                            if count > 0 {
                                word = tmp.as_str().to_string();
                            } else {
                                word = String::from(c);
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
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
    fn part_two_sample_test() {
        let input = read_lines("sample_two.txt").unwrap();
        assert_eq!(281, process(input));
    }
}
