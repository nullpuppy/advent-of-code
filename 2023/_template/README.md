# Advent of Code Daily Solution Template

Base structure to quickly copy for each day's challenge.

## How to use

```shell
# Make a copy of this dir named for the day of the challenge
cp -a _template day-XX
# Fix the Cargo.toml project name.
sed -i "s/{{project-name}}/day-XX" day-XX/Cargo.toml 
```

Then edit `src/bin/part1.rs` for part 1, and `src/bin/part2.rs` for part 2.

`src/bin/common.rs` exists for anything you want to share between parts 1 and 2.

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