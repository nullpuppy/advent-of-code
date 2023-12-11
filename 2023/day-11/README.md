# Advent of Code Daily Solution for Day 11

### Problem:

Parse field that represents an image of space. '.'s are empty space '#'s are
galaxies. To complicate matters, we're given direction that empty rows or
columns indicate that space is compressed, and should actually represent
additional space.

### Part 1 notes

I initially just parsed to tuples, and then built pairs of galaxies, then
mapped the pairs to compute the [taxicab](https://en.wikipedia.org/wiki/Taxicab_geometry)
distance and summed the results. I then went in and refactored everything
based on part2 requirements to improve testability.

### Part 2 notes

This is really just the same as part 1, each empty row or column now represents
orders of magnitude more space. Instructions indicated empty space should be 
1,000,000 times bigger, but somehow this was off-by-one, which is why magnitude
is decremented by 1 before multiplying with the count and being added to the
necessary coordinate.

In part 1, I did the row expansion in-line with the parsing. In order to make
testing easier/cleaner, I added tracking of rows that are missing galaxies
as well as columns. Naming kind of sucks, but it works.

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