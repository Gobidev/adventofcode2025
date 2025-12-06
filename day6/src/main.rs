type ParsedInput = (Vec<Vec<usize>>, Vec<Vec<Option<u32>>>, Vec<char>);

fn parse(input: &str) -> ParsedInput {
    (
        input
            .trim()
            .lines()
            .rev()
            .skip(1)
            .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
            .collect(),
        input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10)).collect())
            .collect(),
        input
            .trim()
            .lines()
            .next_back()
            .unwrap()
            .split_whitespace()
            .map(|c| c.chars().next().unwrap())
            .collect(),
    )
}

fn part1(input: &ParsedInput) -> usize {
    (0..input.0[0].len())
        .map(|c_idx| match input.2[c_idx] {
            '*' => (0..input.0.len()).fold(1, |acc, r_idx| acc * input.0[r_idx][c_idx]),
            _ => (0..input.0.len()).fold(0, |acc, r_idx| acc + input.0[r_idx][c_idx]),
        })
        .sum()
}

fn part2(input: &ParsedInput) -> usize {
    let mut c_idx = 0;
    let mut op_idx = 0;
    let mut num;
    let mut res = 0;
    while op_idx < input.2.len() {
        let mut acc: usize = if input.2[op_idx] == '*' { 1 } else { 0 };
        while c_idx != input.1[0].len() {
            num = 0;
            for r_idx in 0..input.1.len() {
                if let Some(d) = input.1[r_idx][c_idx] {
                    num = 10 * num + d as usize;
                }
            }
            c_idx += 1;
            if num == 0 {
                break;
            }
            match input.2[op_idx] {
                '*' => acc *= num,
                _ => acc += num,
            }
        }
        op_idx += 1;
        res += acc;
    }
    res
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
