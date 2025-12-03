use std::{fs::File, io::{BufRead, BufReader}};


fn array_magic<I>(a: I) -> u32 where I: Iterator<Item = u8> {
    let mut tracker: Vec<(usize, u8)> = a.enumerate().collect();
    tracker.sort_by(|a,b| b.1.cmp(&a.1));
    if tracker[0].0 == tracker.len() - 1 {
        tracker.swap(0, 1);
    };
    let first_index = tracker[0].0;
    // Filter elements whose index is greater than or equal to the first
    let mut tracker= tracker.iter().filter(|x| x.0 >= first_index);
    (tracker.next().unwrap().1 * 10 + tracker.next().unwrap().1) as u32
}


// Grab the highest, then grab the next highest to the right
fn main() {
    let input = File::open("input.txt").unwrap();
    let input_lines = BufReader::new(input).lines();
    let result = input_lines.fold(0u32, |acc, line| acc + array_magic(line.unwrap().chars().map(|e| e.to_digit(10).unwrap() as u8)));
    println!("Result: {result}");
}
