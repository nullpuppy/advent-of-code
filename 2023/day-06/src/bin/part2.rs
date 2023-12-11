#![allow(unused_imports)]
#![allow(dead_code)]
use utils::read_lines;
use day_06::{concat_vec, parse_line};

fn main() {
    let input = read_lines("input.txt").expect("Unable to open input");
    let ans = process(input);
    println!("Part 2: {}", ans);
}

fn process(mut input: impl Iterator<Item = String>) -> usize {
    let time = concat_vec(parse_line(input.next().unwrap()));
    let distance = concat_vec(parse_line(input.next().unwrap()));

    let mut delay = time / 2;

    // while distance traveled is winning, search backwards.
    while delay * (time - delay) > distance {
        delay -= delay / 2;
    }

    // delay now doesn't win, now find first occurrence that we wine
    while delay * (time - delay) < distance {
        delay += 1;
    }

    let first_winning = delay;

    // Now search backwards until we get to the first winning scenario.
    delay = time;

    // delay now doesn't win.
    while delay * (time - delay) < distance {
        delay -= 1;
    }

    // We should be one past the last winning race
    let last_winning = delay;

    last_winning - first_winning + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_test() {
        let input = read_lines("sample.txt").expect("Unable to open sample text");
        assert_eq!(71503, process(input));
    }
}
