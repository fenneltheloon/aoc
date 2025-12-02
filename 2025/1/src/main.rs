use std::fs::File;
use std::io::{BufReader, BufRead};
use std::ops::{Add, Sub};

struct P100 {
    value: u8,
}

impl Add<u32> for P100 {
    type Output = P100;

    fn add(self, rhs: u32) -> Self::Output {
        Self {
            value: ((self.value as u32 + rhs) % 100) as u8,
        }
    }
}

impl Sub<u32> for P100 {
    type Output = P100;

    fn sub(self, rhs: u32) -> Self::Output {
        Self {
            value: ((self.value as u32 - rhs) % 100) as u8,
        }
    }
}

impl P100 {
    fn adjust(&mut self, inc: &str) {
        let (direction, amount) = inc.split_at(1);
        let amount = amount.parse::<u32>().unwrap();
        *self = match direction {
            "R" => self + amount,
            "L" => self - amount,
            _ => panic!("Unexpected input: {inc}"),
        }
    }
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let input_lines = BufReader::new(input).lines();
    let mut zero_count = 0u32;
    let mut current = P100 { value: 50 };
    for line in input_lines {
        let line = line.unwrap();
        current.adjust(&line);
        if current.value == 0 {
            zero_count += 1;
        }
    }
    println!("Final password: {zero_count}");
}
