use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = File::open("input.txt").unwrap();
    let lines = BufReader::new(input).lines();

    let mut lines: Vec<Vec<String>> = lines
        .map(|f| {
            f.unwrap()
                .to_owned()
                .split_whitespace()
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .collect();
    println!("{lines:?}");

    let operations = lines.pop().unwrap();

    let lines = lines
        .iter()
        .map(|e| {
            e.iter()
                .map(|f| f.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    assert_eq!(operations.len(), lines[0].len());

    let mut answer = 0u64;
    // Group all by column
    for i in 0..lines[0].len() {
        let (is_mul, operation) = match operations[i].as_str() {
            "+" => (false, u64::strict_add as fn(u64, u64) -> u64),
            "*" => (true, u64::strict_mul as fn(u64, u64) -> u64),
            _ => panic!(),
        };

        let partial = lines
            .iter()
            .clone()
            .fold(is_mul as u64, |acc, e| operation(acc, e[i]));

        println!("Partial {i}: {partial}");
        answer += partial;
    }

    println!("Answer: {answer}");
}
