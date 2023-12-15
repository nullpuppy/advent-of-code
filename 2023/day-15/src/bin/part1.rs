use day_15::*;
use utils::read_lines;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();

    let input = read_lines("day-15/input.txt").expect("Unable to open input");
    let ans = process(input);
    println!("Part 1: {}", ans);

    Ok(())
}

fn process(input: impl Iterator<Item = String>) -> usize {
    input
        .map(|line| line.split(',').map(hash).reduce(|a, b| a + b).unwrap())
        .reduce(|a, b| a + b)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_test() {
        let input = read_lines("sample.txt").expect("Unable to open sample text");
        assert_eq!(1320, process(input));
    }
}
