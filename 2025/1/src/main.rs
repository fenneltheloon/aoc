use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};

#[derive(Debug)]
struct P100 {
    value: u8,
}

impl Add<i32> for P100 {
    type Output = P100;

    fn add(self, rhs: i32) -> Self::Output {
        Self {
            value: (self.value as i32 + rhs).rem_euclid(100) as u8,
        }
    }
}

impl Sub<i32> for P100 {
    type Output = P100;

    fn sub(self, rhs: i32) -> Self::Output {
        Self {
            value: (self.value as i32 - rhs).rem_euclid(100) as u8,
        }
    }
}

fn adjust(curr: P100, inc: &str) -> P100 {
    let (direction, amount) = inc.split_at(1);
    let amount = amount.parse::<i32>().unwrap();
    match direction {
        "R" => curr + amount,
        "L" => curr - amount,
        _ => panic!("Unexpected input: {inc}"),
    }
}

fn adjust2(mut curr: P100, inc: &str, mut zero_count: u32) -> (P100, u32) {
    let (direction, amount) = inc.split_at(1);
    let amount = amount.parse::<u32>().unwrap();
    // Figure out how many times we pass zero
    match direction {
        "R" => {
            zero_count += (curr.value as u32 + amount) / 100;
            (curr + amount as i32, zero_count)
        }
        "L" => {
            if curr.value == 0 {
                curr.value = 100;
            }
            let delta = curr.value as i32 - amount as i32;
            if delta <= 0 {
                zero_count += 1;
            }
            zero_count += (delta / 100).abs() as u32;
            (curr - amount as i32, zero_count)
        }
        _ => panic!("Unexpected input: {inc}"),
    }
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let input_lines = BufReader::new(input).lines();
    let mut zero_count = 0u32;
    let mut current = P100 { value: 50 };
    for line in input_lines {
        let line = line.unwrap();
        (current, zero_count) = adjust2(current, &line, zero_count);
        println!("{line}: {current:?}, {zero_count}");
        // if current.value == 0 {
        //     zero_count += 1;
        // }
    }
    println!("Final password: {zero_count}");
}
