use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut lines = BufReader::new(input)
        .lines()
        .map(|e| e.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut answer = 0u32;

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            match lines[i][j] {
                'S' => {
                    if lines[i + 1][j] == '.' {
                        lines[i + 1][j] = '|';
                    }
                }
                '.' => {
                    if i > 0 && lines[i - 1][j] == '|' {
                        lines[i][j] = '|';
                    }
                }
                '^' => {
                    if i > 0 && lines[i - 1][j] == '|' {
                        answer += 1;
                        if j > 0 && lines[i][j - 1] == '.' {
                            lines[i][j - 1] = '|';
                        }
                        if j < lines[i].len() && lines[i][j + 1] == '.' {
                            lines[i][j + 1] = '|';
                        }
                    }
                }
                '|' => (),
                e => panic!("Unexpected character {e} in input"),
            }
        }
    }

    for line in lines {
        for c in line {
            print!("{c}");
        }
        println!();
    }

    println!("Answer: {answer}");
}
