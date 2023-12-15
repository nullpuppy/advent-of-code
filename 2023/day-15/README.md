# Advent of Code Day 15 Solution

### [Problem](https://adventofcode.com/2023/day/15)

Implement a very basic hash algorithm.

### Part 1 Notes

No real notes for part 1. Just parse the input, split each line by comma, hash the value, and sum.

### Part 2 Notes

I feel like this probably should have been part 1. This was also pretty simple. We're just
implementing our own hashmap. I just used an array of deques. Given the removal requirements, trees
or linked lists might've been a better idea, but the data we're dealing with is small enough
that a deque seemed good enough.

Nothing else to say really. Allocating static arrays with non-copy values could be better.

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