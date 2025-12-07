use ahash::AHashMap;

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Start,
    Splitter,
    Empty,
}
use Tile::*;

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            'S' => Start,
            '^' => Splitter,
            _ => Empty,
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|l| l.chars().map(Tile::from).collect())
        .collect()
}

fn simulate(input: &[Vec<Tile>]) -> AHashMap<(usize, usize), usize> {
    let mut beam_positions = AHashMap::new();
    beam_positions.insert((0, input[0].iter().position(|t| *t == Start).unwrap()), 1);

    for (r_idx, row) in input.iter().enumerate().skip(1) {
        for (c_idx, tile) in row.iter().enumerate() {
            if !beam_positions.contains_key(&(r_idx - 1, c_idx)) {
                continue;
            }
            let above = beam_positions[&(r_idx - 1, c_idx)];
            match tile {
                Empty => {
                    beam_positions
                        .entry((r_idx, c_idx))
                        .and_modify(|v| *v += above)
                        .or_insert_with(|| above);
                }
                _ => {
                    beam_positions
                        .entry((r_idx, c_idx - 1))
                        .and_modify(|v| *v += above)
                        .or_insert_with(|| above);
                    beam_positions
                        .entry((r_idx, c_idx + 1))
                        .and_modify(|v| *v += above)
                        .or_insert_with(|| above);
                }
            }
        }
    }
    beam_positions
}

fn part1(input: &[Vec<Tile>], beam_positions: &AHashMap<(usize, usize), usize>) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(r_idx, r)| {
            r.iter()
                .enumerate()
                .filter(|(c_idx, t)| {
                    **t == Splitter && beam_positions.contains_key(&(r_idx - 1, *c_idx))
                })
                .count()
        })
        .sum()
}

fn part2(input: &[Vec<Tile>], beam_positions: &AHashMap<(usize, usize), usize>) -> usize {
    beam_positions
        .iter()
        .filter(|((r, _), _)| *r == input.len() - 1)
        .map(|(_, v)| v)
        .sum()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    let beam_positions = simulate(&input);
    println!("{}", part1(&input, &beam_positions));
    println!("{}", part2(&input, &beam_positions));
}
