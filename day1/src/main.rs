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

fn part1(input: &[(char, isize)]) -> isize {
    input
        .iter()
        .fold((0, 50), |(res, dial), instr| {
            (
                if dial == 0 { res + 1 } else { res },
                turn(instr.0, dial, instr.1),
            )
        })
        .0
}

fn part2(input: &[(char, isize)]) -> isize {
    let mut dial_pos: isize = 50;
    let mut res = 0;
    for instruction in input {
        for _ in 0..(instruction.1) {
            dial_pos = turn(instruction.0, dial_pos, 1);
            if dial_pos == 0 {
                res += 1;
            }
        }
    }
    res
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
