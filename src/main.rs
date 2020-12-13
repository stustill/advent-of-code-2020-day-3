use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut input_file = File::open("input.txt")?;
    let mut input = String::new();
    let _ = input_file.read_to_string(&mut input)?;
    let lines: Vec<&str> = input.lines().collect();
    // let slopes = [(3, 1)];
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let result = slopes
        .iter()
        .map(|(right, down)| evaluate(&lines, *right, *down))
        .fold(1, |acc, elem| acc * elem);
    println!("{}", result);
    return Ok(());
}

fn evaluate(lines: &Vec<&str>, right: usize, down: usize) -> usize {
    let mut horizontal_position = 0;
    let mut count = 0;
    for line in lines.iter().step_by(down) {
        if line.chars().nth(horizontal_position) == Some('#') {
            count += 1;
        }
        horizontal_position += right;
        horizontal_position %= line.len();
    }
    count
}
