type ParsedInput = (Vec<(usize, usize)>, Vec<usize>);

fn parse(input: &str) -> ParsedInput {
    let mut i = input.trim().split("\n\n");
    (
        i.next()
            .unwrap()
            .lines()
            .map(|l| {
                let mut nums = l.split('-').map(|num| num.parse().unwrap());
                (nums.next().unwrap(), nums.next().unwrap())
            })
            .collect(),
        i.next()
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect(),
    )
}

fn part1(input: &ParsedInput) -> usize {
    input
        .1
        .iter()
        .filter(|id| {
            input
                .0
                .iter()
                .any(|range| range.0 <= **id && id <= &&range.1)
        })
        .count()
}

fn elliminate_overlapping(ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut new_ranges = vec![];
    let mut current = ranges.to_vec();
    while new_ranges != current {
        new_ranges = current.to_vec();
        for i1 in 0..current.len() {
            for i2 in i1 + 1..current.len() {
                if current[i1].0 <= current[i2].1 && current[i2].0 <= current[i1].1 {
                    current.push((
                        usize::min(current[i1].0, current[i2].0),
                        usize::max(current[i1].1, current[i2].1),
                    ));
                    current.remove(i2);
                    current.remove(i1);
                    break;
                }
            }
        }
    }
    new_ranges
}

fn part2(input: &ParsedInput) -> usize {
    elliminate_overlapping(&input.0)
        .iter()
        .map(|(r0, r1)| (r1 - r0) + 1)
        .sum()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
