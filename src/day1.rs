use anyhow::Result;
use regex::Regex;

const PROBLEM: &str = include_str!("../input/day1.txt");

pub fn solution1() -> Result<()> {
    let sum = PROBLEM.split("\n").map(|line| {
        let mut first_digit: u32 = 0;
        let mut second_digit: u32 = 0;
        for c in line.chars() {
            if c.is_numeric() {
                first_digit = c.to_digit(10).unwrap();
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_numeric() {
                second_digit = c.to_digit(10).unwrap();
                break;
            }
        }

        10*first_digit + second_digit
    }).reduce(|x1, x2| x1 + x2).unwrap();

    println!("Answer: {}", sum);
    Ok(())
}
