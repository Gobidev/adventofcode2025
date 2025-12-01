fn parse(input: &str) -> Vec<(char, isize)> {
    input
        .lines()
        .map(|l| {
            (
                l.chars().next().unwrap(),
                l.chars().skip(1).collect::<String>().parse().unwrap(),
            )
        })
        .collect()
}

fn turn(dir: char, dial_pos: isize, turn_count: isize) -> isize {
    match dir {
        'R' => (dial_pos + turn_count).rem_euclid(100),
        _ => (dial_pos - turn_count).rem_euclid(100),
    }
}

fn part1(input: &[(char, isize)]) -> usize {
    input
        .iter()
        .scan(50, |dial, instr| {
            *dial = turn(instr.0, *dial, instr.1);
            Some(*dial)
        })
        .filter(|dial| *dial == 0)
        .count()
}

fn part2(input: &[(char, isize)]) -> usize {
    input
        .iter()
        .flat_map(|instr| (0..instr.1).map(|_| (instr.0, 1)))
        .scan(50, |dial, instr| {
            *dial = turn(instr.0, *dial, instr.1);
            Some(*dial)
        })
        .filter(|dial| *dial == 0)
        .count()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
