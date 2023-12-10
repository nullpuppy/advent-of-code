#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashSet;
use day_10::{Coord2d, Dim2d, Direction, Grid, Tile};
use utils::read_lines;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();

    let input = read_lines("input.txt").expect("Unable to open input");
    let ans = process(input);
    println!("Part 2: {}", ans);
}

fn process(input: impl Iterator<Item = String>) -> usize {
    let grid = Grid::from_input(input);
    // let loop_coords = grid.loop_coords();
    // let mut inside: HashSet<Coord2d> = HashSet::new();
    //
    // // Start node has exits in 2 directions.
    // // pick one of the directions, then start traversing the loop.
    // // at each point, if not a bend, search in a direction that is contained by loop
    // // until we hit a loop segment. anything not loop is in loop.
    // // continue until we get back to our start position.
    //
    // let exits = grid.get_exits(&grid.start);
    // println!("First exit from {} is {:?}", grid.start, exits[0]);
    // println!("So we want to follow path starting at {}, and going {} but look at tiles going {}", exits[0].0, exits[0].1, exits[1].1);
    // println!("If we end up at a bend, we update traversal_dir and search dir");
    //
    // let mut search_dir = exits[1].1;
    // let mut traverse_dir = exits[0].1;
    // let mut traversal_coord = exits[0].0;
    // let bounds = Dim2d {
    //     height: grid.tiles.len(),
    //     width: grid.tiles[0].len(),
    // };
    //
    // while traversal_coord != grid.start {
    //     let mut search_coord = traversal_coord;
    //     let mut cnt = 0;
    //     loop {
    //         if let Some(next) = search_coord.adjust(&search_dir, &bounds) {
    //             search_coord = next;
    //         } else {
    //             println!("Next coord is None, breaking out.");
    //             break;
    //         }
    //
    //         println!("Checking {} going {} {}", search_coord, search_dir, loop_coords.contains(&search_coord));
    //         cnt += 1;
    //         if loop_coords.contains(&search_coord) {
    //             println!("Breaking out of loop, found loop coord");
    //             break;
    //         }
    //         inside.insert(search_coord);
    //     }
    //
    //     traversal_coord = traversal_coord.adjust(&traverse_dir, &bounds).unwrap();
    //     if grid.tile_at(&traversal_coord).is_bend() {
    //         println!("Adjusting Traverse {}, search {}", traverse_dir, search_dir);
    //         match traverse_dir {
    //             Direction::North => {
    //                 match grid.tile_at(&traversal_coord) {
    //                     Tile::SWBend => {
    //                         traverse_dir = Direction::West;
    //                         search_dir = Direction::South;
    //                     },
    //                     Tile::SEBend => {
    //                         traverse_dir = Direction::East;
    //                         search_dir = Direction::South;
    //                     },
    //                     _ => unreachable!()
    //                 }
    //             }
    //             Direction::South => {
    //                 match grid.tile_at(&traversal_coord) {
    //                     Tile::NWBend => {
    //                         traverse_dir = Direction::West;
    //                         search_dir = Direction::North;
    //                     },
    //                     Tile::NEBend => {
    //                         traverse_dir = Direction::East;
    //                         search_dir = Direction::North;
    //                     },
    //                     _ => unreachable!()
    //                 }
    //             }
    //             Direction::West => {
    //                 match grid.tile_at(&traversal_coord) {
    //                     Tile::NEBend => {
    //                         traverse_dir = Direction::North;
    //                         search_dir = Direction::East;
    //                     },
    //                     Tile::SEBend => {
    //                         traverse_dir = Direction::South;
    //                         search_dir = Direction::East;
    //                     },
    //                     _ => unreachable!()
    //                 }
    //             }
    //             Direction::East => {
    //                 match grid.tile_at(&traversal_coord) {
    //                     Tile::NWBend => {
    //                         traverse_dir = Direction::North;
    //                         search_dir = Direction::West;
    //                     },
    //                     Tile::SWBend => {
    //                         traverse_dir = Direction::South;
    //                         search_dir = Direction::West;
    //                     },
    //                     _ => unreachable!()
    //                 }
    //             }
    //         }
    //         println!("To Traverse {}, search {}", traverse_dir, search_dir);
    //     }
    // }

    let inside = grid.contained_coords();

    println!("{}", grid);
    println!("{:?}", grid);
    println!("Coords inside loop: {:?}", inside);
    inside.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two_sample_four_test() {
        let input = read_lines("sample_four.txt").expect("Unable to open sample text");
        assert_eq!(4, process(input));
    }

    #[test]
    fn part_two_sample_five_test() {
        let input = read_lines("sample_five.txt").expect("Unable to open sample text");
        assert_eq!(8, process(input));
    }

    #[test]
    fn part_two_sample_six_test() {
        let input = read_lines("sample_six.txt").expect("Unable to open sample text");
        assert_eq!(10, process(input));
    }
}
