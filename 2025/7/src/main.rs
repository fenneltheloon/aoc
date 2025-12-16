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
    lines.pop();
    let mut space: Vec<Vec<u64>> = vec![vec![0; lines[0].len()]; lines.len()];

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            match lines[i][j] {
                'S' => {
                    if i < space.len() {
                        space[i + 1][j] += 1;
                        lines[i + 1][j] = '|';
                    }
                }
                '.' => {
                    if i > 0 {
                        space[i][j] += space[i - 1][j];
                        if space[i][j] > 0 {
                            lines[i][j] = '|';
                        }
                    }
                }
                '^' => {
                    if j > 0 && i > 0 && (lines[i][j - 1] == '.' || lines[i][j - 1] == '|') {
                        space[i][j - 1] += space[i - 1][j];
                        lines[i][j - 1] = '|';
                    }
                    if j < space[i].len()
                        && i > 0
                        && (lines[i][j + 1] == '.' || lines[i][j + 1] == '|')
                    {
                        space[i][j + 1] += space[i - 1][j];
                        lines[i][j + 1] = '|';
                    }
                }
                '|' => {
                    if i > 0 {
                        space[i][j] += space[i - 1][j];
                    }
                }
                e => panic!("Unexpected character {e} in input"),
            }
        }
    }

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i][j] == '|' {
                print!("{0}", space[i][j] % 10);
            } else {
                print!("{0}", lines[i][j]);
            }
        }
        println!();
    }

    let answer = space[space.len() - 1].iter().sum::<u64>();
    println!("Answer: {answer}");
}
