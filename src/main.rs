use std::fs;

fn get_number(line: &str) -> i32 {
    let mut numbers: Vec<i32> = vec![];
    for c in line.split("") {
        if let Ok(num) = c.parse() {
            numbers.push(num);
        }
    }
    if numbers.len() == 1 {
        return format!("{}{}", numbers[0], numbers[0]).parse().unwrap();
    } else {
        return format!("{}{}", numbers[0], numbers[numbers.len() - 1])
            .parse()
            .unwrap();
    }
}

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();
    let result: i32 = text.lines().map(|line| get_number(line)).sum::<i32>();
    println!("{:?}", result);
}
