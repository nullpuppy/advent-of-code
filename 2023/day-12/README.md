# Advent of Code Daily Solution For Day 12

### Problem:

[Day 12](https://adventofcode.com/2023/day/12)'s problem statement can be summarized as
we have some damaged data that we need to analyze for possible ways that data can be
repaired. Within the records we are given, we have some bad data amongst the good data, 
and then some numbers to indicate groupings or specific data.

### Part 1 notes

I feel like there's probably a way to approach this problem with trees or graphs, but I
was lazy and went for brute force, and was able to get an answer within a second or two.

At some point, I want to come back to this to find a better solution.

### Part 2 notes

The brute force solution used in part1 is nowhere near fast enough for part 2.
I attempted to run that solution on the test input, but gave up after 17 minutes.

Brute force might eventually work, but this needs to be revisited with an alternate solution.

The [Reddit thread](https://www.reddit.com/r/adventofcode/comments/18ge41g/2023_day_12_solutions/) for solutions seems to indicate dynamic programming solutions.

## Running Solutions

Update `input.txt` and `sample.txt` (or whatever other relevant input and sample txt files)
with the data from the relevant advent-of-code challenge.

```shell
cargo run --bin part1 # To run solution for part1
cargo run --bin part2 # To run solution for part1
```

There are also tests:
```shell
cargo test --bin part1
cargo test --bin part2
```