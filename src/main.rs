use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file_path = "src/advent.txt";
    let file = File::open(file_path)?;

    let mut sum = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            if let Some((first, last)) = extract_digits(&line) {
                sum += combine_digits(first, last);
            }
        }
    }

    println!("Sum of calibration values: {}", sum);

    Ok(())
}

fn extract_digits(line: &str) -> Option<(u32, u32)> {
    let first_digit = line.chars().find(|c| c.is_digit(10))?.to_digit(10)?;
    let last_digit = line.chars().rev().find(|c| c.is_digit(10))?.to_digit(10)?;

    Some((first_digit, last_digit))
}

fn combine_digits(first: u32, last: u32) -> u32 {
    first * 10 + last
}
