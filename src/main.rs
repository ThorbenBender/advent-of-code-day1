use std::fs;

fn get_number(line: &str) -> i32 {
    let digits: Vec<i32> = line
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    match digits.len() {
        1 => digits[0] * 11,
        _ => digits[0] * 10 + digits.last().unwrap(),
    }
}

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();
    let result: i32 = text.lines().map(|line| get_number(line)).sum::<i32>();
    println!("{:?}", result);
}
