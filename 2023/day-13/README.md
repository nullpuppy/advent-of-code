# Advent of Code Solution for Day 13

### [Problem](https://adventofcode.com/2023/day/13)

The primary challenge in today's problem is finding where reflections occur within small
blocks of ascii art.

### Part 1 notes

The first part wasn't too bad. After parsing in each individual pattern, we just iterate
through windows going vertically and horizontally until we find two consecutive rows or
columns, and then check if rows/columns on either side reflect. If so, we've found a 
reflection! This could probably be implemented recursively without too much effort, but
I just went with a simple iterative approach.

### Part 2 notes

This part makes me feel dumb, because I did a dumb. At first it seemed pretty simple, just
allow for a single error to occur while checking rows/columns. This works, but we then need
to actually modify result the pattern and check search again without the error allowance.

I think I have an idea or two that'll get me to a working solution, but I kept running into
issues and not enough time. To be continued, I hope.

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