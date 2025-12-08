use std::{cmp::Reverse, collections::BinaryHeap};

use disjoint::DisjointSet;
use itertools::Itertools;

type Position = (isize, isize, isize);
type DistanceHeap = BinaryHeap<Reverse<BoxPair>>;

#[derive(Debug, Clone, Eq, PartialEq)]
struct BoxPair {
    pos1: Position,
    pos2: Position,
    distance: isize,
}

impl BoxPair {
    fn new() -> Self {
        Self {
            pos1: (0, 0, 0),
            pos2: (0, 0, 0),
            distance: 0,
        }
    }
}

impl Ord for BoxPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for BoxPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Vec<Position> {
    input
        .lines()
        .map(|l| {
            let mut l_iter = l.split(',');
            (
                l_iter.next().unwrap().parse().unwrap(),
                l_iter.next().unwrap().parse().unwrap(),
                l_iter.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn calc_distances(input: &[Position]) -> DistanceHeap {
    let mut distances = BinaryHeap::new();
    for (pos1, pos2) in input.iter().tuple_combinations::<(&Position, &Position)>() {
        distances.push(Reverse(BoxPair {
            pos1: *pos1,
            pos2: *pos2,
            distance: (pos1.0 - pos2.0).pow(2)
                + (pos1.1 - pos2.1).pow(2)
                + (pos1.2 - pos2.2).pow(2),
        }));
    }
    distances
}

fn part1(input: &[Position], distances: &mut DistanceHeap) -> usize {
    let mut ds = DisjointSet::with_len(input.len());
    for _ in 0..1000 {
        let pair = distances.pop().unwrap().0;
        let idx1 = input.iter().position(|p| p == &pair.pos1).unwrap();
        let idx2 = input.iter().position(|p| p == &pair.pos2).unwrap();
        ds.join(idx1, idx2);
    }
    let mut sizes: Vec<_> = ds.sets().iter().map(|s| s.len()).collect();
    sizes.sort_unstable();
    sizes.iter().rev().take(3).product()
}

fn part2(input: &[Position], distances: &mut DistanceHeap) -> isize {
    let mut ds = DisjointSet::with_len(input.len());
    let mut pair = BoxPair::new();
    while ds.sets().len() > 1 {
        pair = distances.pop().unwrap().0;
        let idx1 = input.iter().position(|p| p == &pair.pos1).unwrap();
        let idx2 = input.iter().position(|p| p == &pair.pos2).unwrap();
        ds.join(idx1, idx2);
    }
    pair.pos1.0 * pair.pos2.0
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    let mut distances = calc_distances(&input);
    println!("{}", part1(&input, &mut distances.clone()));
    println!("{}", part2(&input, &mut distances));
}
