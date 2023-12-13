use utils::read_lines;
use day_12::*;

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
    let mut sum = 0;
    for line in input {
        let mut record = SpringRecord::new(line);
        record.unfold(5);
        sum += record.possible_arrangement_count();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two_sample_test() {
        let input = read_lines("sample.txt").expect("Unable to open sample text");
        assert_eq!(525152, process(input));
    }

    #[test]
    fn record_display_test() {
        let record = SpringRecord::new(".# 1".to_string());
        assert_eq!(".# 1", record.to_string())
    }

    #[test]
    fn unfold_record_test() {
        let mut record = SpringRecord::new(".# 1".to_string());
        record.unfold(5);

        assert_eq!(".#?.#?.#?.#?.# 1,1,1,1,1", record.to_string());
    }
}
