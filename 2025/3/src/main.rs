use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

const JOLTAGE_PACK: usize = 12;

fn cmp(a: &(usize, u8), b: &(usize, u8)) -> Ordering {
    let res1 = a.1.cmp(&b.1);
    if res1 == Ordering::Equal {
        return b.0.cmp(&a.0);
    }
    res1
}

fn array_magic<I>(a: I) -> u64
where
    I: Iterator<Item = u8>,
{
    let tracker: Vec<(usize, u8)> = a.enumerate().collect();
    let tracker_len = tracker.len();
    let mut picked_array: Vec<(usize, u8)> = Vec::with_capacity(JOLTAGE_PACK);

    for i in 0..JOLTAGE_PACK {
        let mut wtracker = tracker.clone();
        match picked_array.iter().rev().next() {
            Some(n) => wtracker = wtracker[n.0 + 1..].to_vec(),
            None => {}
        };
        wtracker.sort_by(|a, b| cmp(b, a));
        // println!("{wtracker:?}");
        let mut j = 1;
        while wtracker[0].0 > tracker_len - (JOLTAGE_PACK - i) {
            assert!(j < wtracker.len());
            wtracker.swap(0, j);
            j += 1;
        }
        let item = wtracker[0];
        picked_array.push(wtracker[0]);
        print!("{item:?}");
    }
    println!();
    assert_eq!(picked_array.len(), JOLTAGE_PACK);

    let cell = picked_array
        .iter()
        .map(|e| e.1)
        .fold(0u64, |acc, e| acc * 10 + e as u64);

    println!("Selecting {cell}");
    cell
}

// Grab the highest, then grab the next highest to the right
fn main() {
    let input = File::open("input.txt").unwrap();
    let input_lines = BufReader::new(input).lines();
    let result = input_lines.fold(0u64, |acc, line| {
        acc + array_magic(line.unwrap().chars().map(|e| e.to_digit(10).unwrap() as u8))
    });
    println!("Result: {result}");
}
