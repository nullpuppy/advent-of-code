use day_14::*;
use utils::read_lines;

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
    let mut platform = Platform::new(
        input
            .map(|line| line.chars().map(Rock::from).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );
    println!("Width: {}, Height: {}", platform.width(), platform.height());
    println!("{:?}", platform);

    platform.tilt();
    platform.total_load()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_test() {
        let input = read_lines("sample.txt").expect("Unable to open sample text");
        assert_eq!(136, process(input));
    }
}
