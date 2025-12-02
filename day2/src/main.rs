fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .trim()
        .split(',')
        .map(|r| {
            let mut nums = r.split('-');
            (
                nums.next().unwrap().parse().unwrap(),
                nums.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn id_is_valid(id: usize) -> bool {
    let id_str = id.to_string();
    id_str[..id_str.len() / 2] != id_str[id_str.len() / 2..]
}

fn id_is_valid2(id: usize) -> bool {
    let id_str = id.to_string();
    for len in 1..id_str.len() / 2 {
        if !id_str.len().is_multiple_of(len) {
            continue;
        }
        if id_str[..len].to_string().repeat(id_str.len() / len) == id_str {
            return false;
        }
    }
    true
}

fn part(input: &[(usize, usize)], part2: bool) -> usize {
    input
        .iter()
        .map(|(r1, r2)| {
            (*r1..=*r2)
                .filter(|id| {
                    !if part2 {
                        id_is_valid2(*id)
                    } else {
                        id_is_valid(*id)
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part(&input, false));
    println!("{}", part(&input, true));
}
