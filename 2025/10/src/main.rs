use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
};

const PARENS: [char; 2] = ['(', ')'];
const BRACES: [char; 2] = ['{', '}'];

/// a: 2D matrix of size (m, m+1)
fn gaussian_elim(a: &mut [Vec<f64>]) -> Vec<f64> {
    println!("{a:#?}");
    let col_size = a.len();
    let row_size = a[0].len();
    // The indexes of the free variables, not their value
    let mut free_vars = vec![];
    let mut round = 0;
    while round < col_size {
        let pivot_index = round + free_vars.len();
        let (index, _) =
            a.iter()
                .enumerate()
                .skip(round)
                .fold((0usize, &vec![f64::MIN; row_size]), |acc, e| {
                    if e.1[pivot_index] > acc.1[pivot_index] {
                        e
                    } else {
                        acc
                    }
                });
        a.swap(round, index);
        if a[round][pivot_index] == 0.0 {
            free_vars.push(pivot_index);
            continue;
        }
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
        round += 1;
    }
    println!("{a:#?}");
    // Back substitute
    // free_vars should be sorted smallest to largest index
    // Strongly assert that all values in our matrix are integers after reduction
    for row in a.iter() {
        for item in row {
            assert_eq!(item.fract(), 0.0);
        }
    }

    let sol_vec = a.iter().map(|e| e[row_size - 1]).collect::<Vec<_>>();
    let mut max_value_free_vars = vec![];
    for var in free_vars.iter() {
        // Get the var-th column
        let button = a.iter().map(|e| e[*var]);
        let max_value = sol_vec
            .iter()
            .zip(button)
            .map(|e| if e.1 == 0.0 { f64::MAX } else { e.0 / e.1 })
            .reduce(|acc, e| if acc <= e { acc } else { e })
            .unwrap();
        max_value_free_vars.push(max_value);
    }
    // Now we need to generate permutations for all free variables
    // We have already asserted that all values in max_value_free_perms are integers, cast *should* be safe
    let free_var_perms = max_value_free_vars
        .iter()
        .map(|e| 0..*e as usize)
        .multi_cartesian_product();

    for free_var_perm in free_var_perms {
        // Solve entire matrix here, starting with bottom row
        let mut var_value: HashMap<usize, usize> = HashMap::new();
        for (index, value) in free_vars.iter().zip(free_var_perm.iter()) {
            var_value.insert(*index, *value).unwrap();
        }
        for row in a.iter().rev() {
            let pivot_index = row.iter().enumerate().find(|e| *e.1 == 0.0).unwrap().0;
            // TODO: Move everything greater than pivot index to RHS (clone value), add solution to hashmap
        }
    }
    todo!()
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
