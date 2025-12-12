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
    if insert_position_low % 2 == 0 {
        a.insert(insert_position_low, low_bound);
        if insert_position_high % 2 == 0 {
            a.insert(insert_position_low + 1, up_bound);
        }
    } else {
        if insert_position_high % 2 == 0 {
            a.insert(insert_position_low, up_bound);
        }
    }
}

fn is_in_range<T: Ord>(a: &Vec<T>, val: T) -> bool {
    match a.binary_search(&val) {
        Ok(_) => true,
        Err(n) => n % 2 == 1,
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

    let answer = input_lines.fold(0u64, |acc, e| {
        acc + is_in_range(&test_area, e.unwrap().parse::<u64>().unwrap()) as u64
    });
    println!("Answer: {answer}");
}
