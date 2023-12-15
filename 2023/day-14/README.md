# Advent of Code Day 14 Solution

### [Problem](https://adventofcode.com/2023/day/14)

I'm not really sure how to sum of the ask in today's challenge. We're given a 2d "map" that
we need to tilt in cardinal directions in order to collect movable entities to an edge, with
the caveat that there are also non-movable entities that the movable ones will run into.

### Part 1 Notes

Making this generic sounded like a pain, so I just went with doing what was needed for part 1
and allowed for tilting north. I processed the map by column, and then by row, going in reverse.

### Part 2 Notes

*sigh* So, I went through a bunch of attempts to get something that would just work for each
direction automatically, but couldn't get what I wanted in a clean way. The current code will
probably work, but is slow. I'm guessing this is another challenge that might benefit from
memoization, or cycle checks. I'll take another look later.

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