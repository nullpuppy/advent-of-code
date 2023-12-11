use utils::read_lines;
use day_03::parse_input;

fn main() {
    let input = read_lines("input.txt").unwrap();
    let ans = process(input);
    println!("Part 2: {}", ans);
}

fn process(input: impl Iterator<Item = String>) -> usize {
    let mut sum = 0;

    // track part numbers, and parts.
    let (symbols, part_numbers) = parse_input(input);

    // Iterate through the parts, and look to see if we can find an adjacent part number.
    // for every adjacent part number, add part number to sum.
    // we only want to add the part the first time it's found.
    for symbol in symbols {
        let found = symbol.adjacent_part_numbers(&part_numbers);

        if found.len() == 2 {
            sum += found
                .iter()
                .map(|part_num| part_num.num)
                .reduce(|l, r| l * r)
                .unwrap();
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
        assert_eq!(467835, process(input));
    }
}
