use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// Delete any bound in the middle of the two being inserted, including overlapping
// Check if bound to left of new one has even index, if so then new bound is in an existing range and can be deleted.
// This is the only allowed insert
fn test_insert<T: Ord>(a: &mut Vec<T>, low_bound: T, up_bound: T) {
    let insert_position_low = match a.binary_search(&low_bound) {
        Ok(n) => n,
        Err(n) => n,
    };
    let insert_position_high = match a.binary_search(&up_bound) {
        Ok(n) => n,
        Err(n) => n,
    };
    a.drain(insert_position_low..insert_position_high);
    let is_new_low = insert_position_low % 2 == 0;
    let is_new_high = insert_position_high % 2 == 0;
    if is_new_low {
        a.insert(insert_position_low, low_bound);
    }
    if is_new_high {
        a.insert(insert_position_low + is_new_low as usize, up_bound);
    }
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut input_lines = BufReader::new(input).lines();
    let mut test_area: Vec<u64> = Vec::new();

    let ranges = input_lines
        .by_ref()
        .take_while(|e| e.as_ref().is_ok_and(|f| f != ""));
    for range in ranges {
        let range = range.unwrap();
        let mut range = range.split("-").map(|f| f.parse::<u64>().unwrap());
        test_insert(&mut test_area, range.next().unwrap(), range.next().unwrap());
    }
    assert!(test_area.is_sorted());
    println!("Range vector: {test_area:?}");

    let answer = test_area
        .iter()
        .tuples::<(_, _)>()
        .fold(0u64, |acc, e| acc + e.1 - e.0 + 1);

    println!("Answer: {answer}");
}
