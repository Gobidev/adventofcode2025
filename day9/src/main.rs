use itertools::Itertools;

type Position = (isize, isize);

fn parse(input: &str) -> Vec<Position> {
    input
        .lines()
        .map(|l| {
            let nums = l.split_once(',').unwrap();
            (nums.0.parse().unwrap(), nums.1.parse().unwrap())
        })
        .collect()
}

fn part1(input: &[Position]) -> isize {
    input
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| ((p1.0 - p2.0).abs() + 1) * ((p1.1 - p2.1).abs() + 1))
        .max()
        .unwrap()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
}
