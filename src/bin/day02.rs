use aoc2020::read_input;

fn main() {
    let passwords = read_input(2);
    let passwords: Vec<&str> = passwords.lines().collect();

    println!("Part 1: {}", part1(&passwords));
    println!("Part 2: {}", part2(&passwords));
}

fn part1(passwords: &[&str]) -> usize {
    passwords
        .iter()
        .filter(|line| {
            let (range, condition, password) = {
                let mut parts = line.splitn(3, ' ');
                (
                    parts.next().unwrap(),
                    parts.next().unwrap(),
                    parts.next().unwrap(),
                )
            };

            let (a, b) = range.split_once('-').unwrap();
            let a: usize = a.parse().unwrap();
            let b: usize = b.parse().unwrap();

            let condition = condition.chars().next().unwrap();
            let count = password.chars().filter(|&c| c == condition).count();

            (a..=b).contains(&count)
        })
        .count()
}

fn part2(passwords: &[&str]) -> usize {
    passwords
        .iter()
        .filter(|line| {
            let (range, condition, password) = {
                let mut parts = line.splitn(3, ' ');
                (
                    parts.next().unwrap(),
                    parts.next().unwrap(),
                    parts.next().unwrap(),
                )
            };

            let (a, b) = range.split_once('-').unwrap();
            let a: usize = a.parse().unwrap();
            let b: usize = b.parse().unwrap();

            let condition = condition.chars().next().unwrap();
            let mut count = 0;

            if password.chars().nth(a-1).unwrap() == condition {
                count += 1;
            }
            if password.len() >= b && password.chars().nth(b-1).unwrap() == condition {
                count += 1;
            }
            count == 1
        })
        .count()
}
