use day_15::*;
use regex::Regex;
use std::collections::VecDeque;
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
    println!("Part 2: {}", ans);

    Ok(())
}

fn process(input: impl Iterator<Item = String>) -> usize {
    // vec of vec of Lens
    let mut hashmap: [VecDeque<Lens>; 256] = [(); 256].map(|_| VecDeque::new());
    let re = Regex::new(r"(?P<label>[^-=]+)(?P<op>[=-])(?P<focal_length>\d*)?")
        .expect("lens parse regex");
    input.for_each(|line| {
        line.split(',').for_each(|step| {
            let caps = re
                .captures(step)
                .expect("failed to process captures for {step}");

            let label = &caps["label"];
            let idx = hash(label);
            match &caps["op"] {
                "=" => {
                    let focal_length = caps["focal_length"].parse().expect("number");

                    if let Some(pos) = hashmap[idx]
                        .iter_mut()
                        .position(|lens| lens.label() == label)
                    {
                        hashmap[idx][pos].set_focal_length(focal_length);
                    } else {
                        let lens = Lens::new(label, focal_length);
                        hashmap[idx].push_back(lens);
                    }
                }
                "-" => {
                    if let Some(pos) = hashmap[idx]
                        .iter_mut()
                        .position(|lens| lens.label() == label)
                    {
                        hashmap[idx].remove(pos);
                    }
                }
                _ => {
                    // no other ops supported
                    unreachable!();
                }
            }
        })
    });

    let mut focusing_power = 0;
    for (box_num, vec) in hashmap.iter().enumerate() {
        for (slot, lens) in vec.iter().enumerate() {
            focusing_power += (box_num + 1) * (slot + 1) * lens.focal_length()
        }
    }
    focusing_power
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_test() {
        let input = read_lines("sample.txt").expect("Unable to open sample text");
        assert_eq!(145, process(input));
    }
}
