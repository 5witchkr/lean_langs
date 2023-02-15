use std::io::stdin;

pub fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let nums = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();
    if nums[0] > nums[1] {
        println!(">");
    } else if nums[0] < nums[1] {
        println!("<");
    } else {
        println!("==");
    }
}
