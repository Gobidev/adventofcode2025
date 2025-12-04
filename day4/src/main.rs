fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c == '@').collect())
        .collect()
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn can_be_removed(input: &[Vec<bool>], l_idx: usize, r_idx: usize) -> bool {
    if !input[l_idx][r_idx] {
        return false;
    }
    DIRECTIONS
        .iter()
        .filter(|dir| {
            input
                .get((l_idx as isize + dir.0) as usize)
                .is_some_and(|l| l.get((r_idx as isize + dir.1) as usize).is_some_and(|r| *r))
        })
        .count()
        < 4
}

fn part1(input: &mut [Vec<bool>]) -> usize {
    (0..input.len())
        .map(|l_idx| {
            (0..input[0].len())
                .filter(|r_idx| {
                    let remove = can_be_removed(input, l_idx, *r_idx);
                    if remove {
                        input[l_idx][*r_idx] = false;
                    }
                    remove
                })
                .count()
        })
        .sum()
}

fn part2(input: &mut [Vec<bool>]) -> usize {
    let mut count = 0;
    let mut removed = 1;
    while removed != 0 {
        removed = part1(input);
        count += removed;
    }
    count
}

fn main() {
    let mut input = parse(include_str!("../input.txt"));
    println!("{}", part1(&mut input.clone()));
    println!("{}", part2(&mut input));
}
