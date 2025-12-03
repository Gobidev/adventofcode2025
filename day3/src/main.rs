fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn find(part: &[u32], remaining: usize) -> Vec<u32> {
    if remaining == 0 {
        return Vec::new();
    }
    let allowed_max_part = &part[..part.len() - (remaining - 1)];
    let max = allowed_max_part.iter().max().unwrap();
    let max_index = allowed_max_part.iter().position(|x| x == max).unwrap();
    let mut res = vec![*max];
    res.append(&mut find(&part[max_index + 1..], remaining - 1));
    res
}

fn part(input: &[Vec<u32>], battery_count: usize) -> usize {
    input
        .iter()
        .map(|l| {
            find(l, battery_count)
                .iter()
                .fold(0, |acc, x| 10 * acc + (*x as usize))
        })
        .sum()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part(&input, 2));
    println!("{}", part(&input, 12));
}
