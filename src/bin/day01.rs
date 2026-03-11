use aoc2020::read_input;

fn main() {
    let numbers: Vec<i32> = read_input(1)
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    if let Some(result) = find_two_sum(&numbers, 2020) {
        println!("Part 1: {result}");
    }

    if let Some(result) = find_three_sum(&numbers, 2020) {
        println!("Part 2: {result}");
    }
}

fn find_two_sum(numbers: &[i32], target: i32) -> Option<i32> {
    for (i, &a) in numbers.iter().enumerate() {
        for &b in &numbers[i + 1..] {
            if a + b == target {
                return Some(a * b);
            }
        }
    }
    None
}

fn find_three_sum(numbers: &[i32], target: i32) -> Option<i32> {
    for (i, &a) in numbers.iter().enumerate() {
        for (j, &b) in numbers[i + 1..].iter().enumerate() {
            for &c in &numbers[i + j + 1..] {
                if a + b + c == target {
                    return Some(a * b * c);
                }
            }
        }
    }
    None
}
