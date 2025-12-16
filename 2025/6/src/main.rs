use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = File::open("input.txt").unwrap();
    let lines = BufReader::new(input).lines();

    let mut char_matrix = lines
        .map(|e| e.unwrap().chars().rev().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut operations = char_matrix.pop().unwrap().into_iter().filter(|e| *e != ' ');

    let mut answer = 0u64;
    let mut transpose: Vec<Vec<char>> = vec![];
    for i in 0..char_matrix[0].len() {
        // println!("Starting transpose: {transpose:?}");
        let mut all_spaces = true;
        transpose.push(vec![]);
        for (j, _) in char_matrix.iter().enumerate() {
            match char_matrix[j][i] {
                e if e.is_ascii_digit() => {
                    let l = transpose.len();
                    transpose[l - 1].push(e);
                    all_spaces = false;
                }
                _ => (),
            }
        }

        if !all_spaces {
            continue;
        }

        transpose.pop();

        let nums = transpose.iter().map(|e| {
            e.iter()
                .fold(0u64, |acc, e| acc * 10 + e.to_digit(10).unwrap() as u64)
        });
        let (acc, operation) = match operations.next().unwrap() {
            '+' => (0u64, u64::strict_add as fn(u64, u64) -> u64),
            '*' => (1u64, u64::strict_mul as fn(u64, u64) -> u64),
            e => panic!("Found unexpected {e} in operations"),
        };

        println!("Nums: {nums:?}");

        let partial = nums.fold(acc, operation);
        println!("Partial: {partial}");
        answer += partial;
        transpose = vec![];
    }

    let nums = transpose.iter().map(|e| {
        e.iter()
            .fold(0u64, |acc, e| acc * 10 + e.to_digit(10).unwrap() as u64)
    });
    let (acc, operation) = match operations.next().unwrap() {
        '+' => (0u64, u64::strict_add as fn(u64, u64) -> u64),
        '*' => (1u64, u64::strict_mul as fn(u64, u64) -> u64),
        e => panic!("Found unexpected {e} in operations"),
    };

    println!("Nums: {nums:?}");

    let partial = nums.fold(acc, operation);
    println!("Partial: {partial}");
    answer += partial;

    println!("Answer: {answer}");
}
