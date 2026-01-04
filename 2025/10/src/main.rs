use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const PARENS: [char; 2] = ['(', ')'];
const BRACES: [char; 2] = ['{', '}'];

/// a: 2D matrix of size (m, m+1)
fn gaussian_elim(a: &mut [Vec<f64>]) -> Vec<f64> {
    println!("{a:#?}");
    let col_size = a.len();
    let row_size = a[0].len();
    let free_vars = vec![];
    for pivot_index in 0..col_size {
        let (index, _) = a.iter().enumerate().skip(pivot_index).fold(
            (0usize, &vec![f64::MIN; row_size]),
            |acc, e| {
                if e.1[pivot_index] > acc.1[pivot_index] {
                    e
                } else {
                    acc
                }
            },
        );
        a.swap(pivot_index, index);
        if a[pivot_index][piv]
        let bottom_factor = a[pivot_index][pivot_index];
        let ref_row = a[pivot_index].clone();
        // Clear out all rows beneath
        for mod_row in a.iter_mut().skip(pivot_index + 1) {
            let top_factor = mod_row[pivot_index];
            for (bottom, top) in mod_row.iter_mut().zip(ref_row.iter()) {
                *bottom -= (top_factor / bottom_factor) * top;
            }
            assert_eq!(mod_row[pivot_index], 0.0);
        }
    }
    println!("{a:#?}");
    // Back substitute
    let mut sol_vec: Vec<f64> = vec![];
    for row_index in (0..col_size).rev() {
        for i in (row_index + 1..col_size).rev() {
            a[row_index][col_size] -= a[row_index][i] * sol_vec[col_size - (row_index + 1)];
        }
        a[row_index][col_size] /= a[row_index][row_index];
        sol_vec.push(a[row_index][col_size]);
    }
    sol_vec.reverse();
    sol_vec
}

fn transpose<T: Copy + Default>(a: &[Vec<T>]) -> Vec<Vec<T>> {
    let m = a.len();
    let n = a[0].len();

    let mut at = vec![vec![T::default(); m]; n];
    for i in 0..m {
        for j in 0..n {
            at[j][i] = a[i][j];
        }
    }
    at
}

fn line_to_joltage(line: String) -> usize {
    println!("{line}");
    let mut space_split = line.split_ascii_whitespace().collect::<Vec<_>>();
    space_split.remove(0);
    let joltages = space_split
        .pop()
        .unwrap()
        .trim_matches(BRACES)
        .split(',')
        .map(|e| e.parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    let size = joltages.len();
    let mut matrix = space_split
        .iter()
        .map(|e| {
            let positions = e
                .trim_matches(PARENS)
                .split(',')
                .map(|f| f.parse::<usize>().unwrap());
            let mut column = vec![0.0; size];
            for position in positions {
                column[position] = 1.0;
            }
            column
        })
        .collect::<Vec<_>>();
    matrix.push(joltages);
    let mut matrix = transpose(&matrix);
    let presses: f64 = gaussian_elim(&mut matrix).iter().sum();

    if presses.fract() != 0.0 {
        panic!("Non-integer number of presses for a line: {presses} {line}");
    }
    print!("{}", presses as usize);
    presses as usize
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let result = BufReader::new(input)
        .lines()
        .map(|e| e.unwrap())
        .fold(0usize, |acc, e| acc + line_to_joltage(e));

    println!("Answer: {result}");
}
