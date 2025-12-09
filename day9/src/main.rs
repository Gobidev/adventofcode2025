use rayon::prelude::*;
use std::{cmp::Reverse, collections::HashMap};

use itertools::{Itertools};

type Position = (isize, isize);

// #[derive(Debug, Clone, Eq, PartialEq)]
// enum Tile {
//     Empty,
//     Red,
//     Green,
// }
// use Tile::*;
//
// impl Display for Tile {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 Empty => ".",
//                 Red => "#",
//                 Green => "X",
//             }
//         )
//     }
// }

fn parse(input: &str) -> Vec<Position> {
    input
        .lines()
        .map(|l| {
            let nums = l.split_once(',').unwrap();
            (nums.0.parse().unwrap(), nums.1.parse().unwrap())
        })
        .collect()
}

// fn reduce_positions(positions: &[Position]) -> Vec<Position> {
//     let min_l = *positions.iter().map(|(_, l)| l).min().unwrap();
//     let min_c = *positions.iter().map(|(c, _)| c).min().unwrap();
//     positions
//         .iter()
//         .map(|(c, l)| (c - min_c, l - min_l))
//         .collect()
// }

// fn construct_grid(input: &[Position]) -> Vec<Vec<Tile>> {
//     let max_l = *input.iter().map(|(_, l)| l).max().unwrap() + 1;
//     let max_c = *input.iter().map(|(c, _)| c).max().unwrap() + 1;
//
//     let mut grid = vec![vec![Empty; max_c as usize]; max_l as usize];
//
//     // perimeter
//     for (red_pos1, red_pos2) in input.iter().zip(input.iter().cycle().skip(1)) {
//         grid[red_pos1.1 as usize][red_pos1.0 as usize] = Red;
//         if red_pos1.0 == red_pos2.0 {
//             for l_idx in isize::min(red_pos1.1, red_pos2.1) + 1..isize::max(red_pos1.1, red_pos2.1)
//             {
//                 grid[l_idx as usize][red_pos1.0 as usize] = Green;
//             }
//         } else {
//             for c_idx in isize::min(red_pos1.0, red_pos2.0) + 1..isize::max(red_pos1.0, red_pos2.0)
//             {
//                 grid[red_pos1.1 as usize][c_idx as usize] = Green;
//             }
//         }
//     }
//
//     // fill greens
//     let mut start_idx = 0;
//     let mut crossed = false;
//     for c_idx in 0..grid[0].len() {
//         if grid[1][c_idx] != Empty {
//             crossed = true;
//         } else if crossed {
//             start_idx = c_idx;
//             break;
//         }
//     }
//     grid = flood(&mut grid, &(start_idx as isize, 1)).to_vec();
//     grid
// }
//
// fn flood<'a>(grid: &'a mut [Vec<Tile>], pos: &Position) -> &'a mut [Vec<Tile>] {
//     if grid[pos.1 as usize][pos.0 as usize] == Empty {
//         grid[pos.1 as usize][pos.0 as usize] = Green;
//         flood(grid, &(pos.0 - 1, pos.1));
//         flood(grid, &(pos.0 + 1, pos.1));
//         flood(grid, &(pos.0, pos.1 - 1));
//         flood(grid, &(pos.0, pos.1 + 1));
//     }
//     grid
// }
//
// fn print_grid(grid: &[Vec<Tile>]) {
//     println!(
//         "{}",
//         grid.iter()
//             .map(|l| l.iter().map(|t| t.to_string()).collect::<String>() + "\n")
//             .collect::<String>()
//     )
// }

// fn part2(input: &[Position], grid: &[Vec<Tile>]) -> isize {
//     input
//         .iter()
//         .tuple_combinations()
//         .map(|(p1, p2)| {
//             // horizontal 1
//             let mut seen_empty = false;
//             for c_idx in isize::min(p1.0, p2.0)..isize::max(p1.0, p2.0) {
//                 if grid[p1.1 as usize][c_idx as usize] == Empty {
//                     seen_empty = true;
//                 } else if seen_empty {
//                     return 0;
//                 }
//             }
//             // horizontal 2
//             let mut seen_empty = false;
//             for c_idx in isize::min(p1.0, p2.0)..isize::max(p1.0, p2.0) {
//                 if grid[p2.1 as usize][c_idx as usize] == Empty {
//                     seen_empty = true;
//                 } else if seen_empty {
//                     return 0;
//                 }
//             }
//             // vertical 1
//             let mut seen_empty = false;
//             for l_idx in isize::min(p1.1, p2.1)..isize::max(p1.1, p2.1) {
//                 if grid[l_idx as usize][p1.0 as usize] == Empty {
//                     seen_empty = true;
//                 } else if seen_empty {
//                     return 0;
//                 }
//             }
//             // vertical 2
//             let mut seen_empty = false;
//             for l_idx in isize::min(p1.1, p2.1)..isize::max(p1.1, p2.1) {
//                 if grid[l_idx as usize][p2.0 as usize] == Empty {
//                     seen_empty = true;
//                 } else if seen_empty {
//                     return 0;
//                 }
//             }
//             ((p1.0 - p2.0).abs() + 1) * ((p1.1 - p2.1).abs() + 1)
//         })
//         .max()
//         .unwrap()
// }

fn part1(input: &[Position]) -> isize {
    input
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| ((p1.0 - p2.0).abs() + 1) * ((p1.1 - p2.1).abs() + 1))
        .max()
        .unwrap()
}

fn is_inside(
    pos: &Position,
    verticals_grid: &[Vec<bool>],
    // cache: &mut HashMap<Position, bool>,
) -> bool {
    // if let Some(res) = cache.get(pos) {
    //     return *res;
    // }
    let mut inside = false;
    let mut upper_corner = false;
    for c_idx in 0..=pos.0 {
        if *verticals_grid
            .get(pos.1 as usize)
            .unwrap_or(&Vec::new())
            .get(c_idx as usize)
            .unwrap_or(&false)
        {
            if !inside {
                upper_corner = *verticals_grid
                    .get(pos.1 as usize + 1)
                    .unwrap_or(&Vec::new())
                    .get(c_idx as usize)
                    .unwrap_or(&false);
                inside = !inside;
            } else if (upper_corner
                && *verticals_grid
                    .get(pos.1 as usize + 1)
                    .unwrap_or(&Vec::new())
                    .get(c_idx as usize)
                    .unwrap_or(&false))
                || (!upper_corner
                    && *verticals_grid
                        .get(pos.1 as usize - 1)
                        .unwrap_or(&Vec::new())
                        .get(c_idx as usize)
                        .unwrap_or(&false))
            {
                inside = !inside;
            }
        }
    }
    // cache.insert(*pos, inside);
    inside
}

fn part2(input: &[Position]) -> isize {
    let mut verticals: HashMap<Position, Position> = HashMap::new();

    for (red_pos1, red_pos2) in input.iter().zip(input.iter().cycle().skip(1)) {
        if red_pos1.0 == red_pos2.0 {
            if red_pos1.1 < red_pos2.1 {
                verticals.insert(*red_pos1, *red_pos2);
            } else {
                verticals.insert(*red_pos2, *red_pos1);
            }
        }
    }
    let max_l = *input.iter().map(|(_, l)| l).max().unwrap() + 1;
    let max_c = *input.iter().map(|(c, _)| c).max().unwrap() + 1;
    let mut verticals_grid = vec![vec![false; max_c as usize]; max_l as usize];
    for (p1, p2) in verticals {
        for l_idx in p1.1..=p2.1 {
            verticals_grid[l_idx as usize][p1.0 as usize] = true;
        }
    }
    // let mut cache = HashMap::new();
    let mut all_pairs: Vec<(Position, Position)> = input
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| (*p1, *p2))
        .collect();
    all_pairs.sort_unstable_by_key(|(p1, p2)| Reverse((p1.0 - p2.0).abs()));
    let l_idx_candidate1 = all_pairs[0].0.1;
    let l_idx_candidate2 = all_pairs[1].0.1;

    let mut filtered_pairs: Vec<_> = all_pairs
        .iter()
        .filter(|(p1, p2)| {
            p1.1 == l_idx_candidate1 || p1.1 == l_idx_candidate2 || p2.1 == l_idx_candidate1 || p2.1 == l_idx_candidate2
        })
        .collect();
    filtered_pairs.sort_unstable_by_key(|(p1, p2)| Reverse(((p1.0 - p2.0).abs() + 1) * ((p1.1 - p2.1).abs() + 1)));

    let winner = filtered_pairs
        .iter()
        .find(|(p1, p2)| {
            // horizontal 1
            for c_idx in isize::min(p1.0, p2.0)..isize::max(p1.0, p2.0) {
                if !is_inside(&(c_idx, p1.1), &verticals_grid) {
                    return false;
                }
            }
            // horizontal 2
            for c_idx in isize::min(p1.0, p2.0)..isize::max(p1.0, p2.0) {
                if !is_inside(&(c_idx, p2.1), &verticals_grid) {
                    return false;
                }
            }
            // vertical 1
            for l_idx in isize::min(p1.1, p2.1)..isize::max(p1.1, p2.1) {
                if !is_inside(&(p1.0, l_idx), &verticals_grid) {
                    return false;
                }
            }
            // vertical 2
            for l_idx in isize::min(p1.1, p2.1)..isize::max(p1.1, p2.1) {
                if !is_inside(&(p2.0, l_idx), &verticals_grid) {
                    return false;
                }
            }
            true
        })
        .unwrap();
    // .sorted_by(|p1: Po, p2| ((p1.0 - p2.0).abs() + 1) * ((p1.1 - p2.1).abs() + 1))
    // .rev()
    // .find(|(p1, p2)| {
    // })
    // .unwrap();

    ((winner.0.0 - winner.1.0).abs() + 1) * ((winner.0.1 - winner.1.1).abs() + 1)
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
    // let grid = construct_grid(&input);
    // print_grid(&grid);

    // runs for about 45min :)
    println!("{}", part2(&input));
}
