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
    println!("Part 1: {}", ans);
    Ok(())
}

fn process(input: impl Iterator<Item = String>) -> usize {
    let mut cols = 0;
    let mut rows = 0;
    let patterns = parse_input_to_patterns(input);

    for pattern in patterns.iter() {
        let edge = pattern.reflection_line(false);
        rows += edge.0.unwrap_or(0);
        cols += edge.1.unwrap_or(0);
    }

    rows * 100 + cols
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_test() {
        let input = read_lines("sample.txt").expect("Unable to open sample text");
        assert_eq!(405, process(input));
    }

    #[test]
    fn string_to_pattern_test() {
        let input = ["#...##..#", "#....#..#", "..##..###", "#####.##.", "#####.##.", "..##..###", "#....#..#",];

        let mut pattern = vec![];
        for line in input.iter() {
            pattern.push(
            line
                .chars()
                .map(Terrain::from)
                .collect());
        }
        let pattern = AshAndRockPattern::new(pattern);
        assert_eq!(7, pattern.height());
        assert_eq!(9, pattern.width());
    }

}
