use utils::read_lines;
use day_13::*;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();

    let input = read_lines("input.txt").expect("Unable to open input");
    let ans = process(input);
    println!("Part 2: {}", ans);
    Ok(())
}

fn process(input: impl Iterator<Item = String>) -> usize {
    let mut cols = 0;
    let mut rows = 0;
    let mut patterns = parse_input_to_patterns(input);

    for pattern in patterns.iter() {
        let edge = pattern.reflection_line(true);
        rows += edge.0.unwrap_or(0);
        cols += edge.1.unwrap_or(0);
    }

    rows * 100 + cols
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two_sample_test() {
        let input = read_lines("sample.txt").expect("Unable to open sample text");
        assert_eq!(400, process(input));
    }
}
