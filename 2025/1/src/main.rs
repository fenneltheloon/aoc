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
fn main() {
    let input = File::open("input.txt").unwrap();
    let input_lines = BufReader::new(input).lines();
    let mut zero_count = 0u32;
    let mut current = P100 { value: 50 };
    for line in input_lines {
        let line = line.unwrap();
        current = adjust(current, &line);
        println!("{line}: {current:?}");
        if current.value == 0 {
            zero_count += 1;
        }
    }
    println!("Final password: {zero_count}");
}
