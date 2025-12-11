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

fn single_row(buffer: &Vec<Vec<char>>, mod_buffer: &mut Vec<Vec<char>>, row_index: usize) -> u32 {
    let mut sum = 0u32;
    let mut new_row = buffer[row_index].clone();
    for column_index in 0..buffer[row_index].len() {
        if *buffer[row_index].get(column_index).unwrap() == '.' {
            continue;
        }
        let mut adjs = 0u8;
        if column_index > 0 {
            adjs += check_char(buffer[row_index].get(column_index - 1));
            adjs += check_char(buffer[row_index - 1].get(column_index - 1));
            adjs += check_char(buffer[row_index + 1].get(column_index - 1));
        }
        adjs += check_char(buffer[row_index].get(column_index + 1));
        adjs += check_char(buffer[row_index - 1].get(column_index));
        adjs += check_char(buffer[row_index - 1].get(column_index + 1));
        adjs += check_char(buffer[row_index + 1].get(column_index));
        adjs += check_char(buffer[row_index + 1].get(column_index + 1));
        if adjs < 4 {
            sum += 1;
            new_row[column_index] = '.';
        }
    }
    mod_buffer.push(new_row);
    sum
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let input_lines = BufReader::new(input).lines();
    let mut buffer: Vec<Vec<char>> = Vec::new();
    buffer.extend(input_lines.map(|e| e.unwrap().chars().collect()));
    buffer.insert(0,vec!['.'; buffer[0].len()]);
    buffer.push(vec!['.'; buffer[0].len()]);

    let buff_len = buffer.len() - 2;
    let mut is_modified = true;
    let mut answer = 0u32;

    let mut round = 1u32;
    while is_modified {
        println!("Round {round}");
        is_modified = false;
        let mut mod_buffer  = Vec::with_capacity(buffer.len());
        mod_buffer.push(vec!['.'; buffer[0].len()]);
        for row_index in 1..=buff_len {
            let single =  single_row(&buffer, &mut mod_buffer, row_index);
            if single > 0 {
                is_modified = true;
            }
            println!("Row {row_index}: removed {single}");
            answer += single;       
        }
        mod_buffer.push(vec!['.'; buffer[0].len()]);
        buffer = mod_buffer;
        round += 1;
    }

    println!("Result: {answer}");
}
