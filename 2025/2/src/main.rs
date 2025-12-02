use std::{fs::File, io::Read};

fn main() {
    let mut input = File::open("input.txt").unwrap();
    let mut input_string = String::new();
    let _ = input.read_to_string(&mut input_string).unwrap();
    let input_string = input_string.trim();
    let input_iter = input_string.split(",");
    let mut sum = 0u64;

    for range in input_iter {
        let split: Vec<&str> = range.split("-").collect();
        let left_bound = match split[0].parse::<u64>() {
            Ok(n) => n,
            Err(e) => panic!("{e}: {split:?}")
        };
        let right_bound = match split[1].parse::<u64>() {
            Ok(n) => n,
            Err(e) => panic!("{e}: {split:?}")
        };
        for num in left_bound..=right_bound {
            let num_array: Vec<char> = num.to_string().chars().collect();
            if num_array.len() % 2 == 1 {
                continue;
            }
            if num_array[..num_array.len() / 2] == num_array[num_array.len() / 2..] {
                sum += num;
                println!("Adding {num}");
            }
        }
    }
    println!("Final sum: {sum}")
}
