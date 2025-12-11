use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn check_char(c: Option<&char>) -> u8 {
    match c {
        Some(d) => match d {
            '@' => 1,
            _ => 0,
        },
        None => 0,
    }
}

fn single_row(buffer: &Vec<Vec<char>>) -> u32 {
    let mut sum = 0u32;
    for i in 0..buffer[1].len() {
        if *buffer[1].get(i).unwrap() == '.' {
            continue;
        }
        let mut adjs = 0u8;
        if i > 0 {
            adjs += check_char(buffer[1].get(i - 1));
            adjs += check_char(buffer[0].get(i - 1));
            adjs += check_char(buffer[2].get(i - 1));
        }
        adjs += check_char(buffer[1].get(i + 1));
        adjs += check_char(buffer[0].get(i));
        adjs += check_char(buffer[0].get(i + 1));
        adjs += check_char(buffer[2].get(i));
        adjs += check_char(buffer[2].get(i + 1));
        if adjs < 4 {
            sum += 1;
        }
    }
    sum
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut input_lines = BufReader::new(input).lines();
    let mut buffer: Vec<Vec<char>> = Vec::new();

    // Setup buffer
    buffer.extend(
        input_lines
            .by_ref()
            .take(2)
            .map(|e| e.unwrap().chars().collect()),
    );
    let length = buffer[0].len();
    let row_padding = vec!['.'; length];
    buffer.insert(0, row_padding);
    let mut answer = 0u32;
    let sr = single_row(&buffer);
    println!("Row: {sr}");
    answer += sr;

    for line in input_lines {
        buffer.remove(0);
        buffer.push(line.unwrap().chars().collect());
        let sr = single_row(&buffer);
        println!("Row: {sr}");
        answer += sr;
    }

    buffer.remove(0);
    buffer.push(vec!['.'; length]);
    let sr = single_row(&buffer);
    println!("Row: {sr}");
    answer += sr;

    println!("Result: {answer}");
}
