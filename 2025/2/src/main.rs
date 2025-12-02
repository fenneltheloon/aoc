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
        'a: for num in left_bound..=right_bound {
            let num_array: Vec<char> = num.to_string().chars().collect();
            let array_len = num_array.len();
            for chunk_size in 1..=array_len / 2 {
                let mut chunks = num_array.chunks(chunk_size);
                let first = chunks.next().unwrap();
                if chunks.all(|e| e == first) {
                    sum += num;
                    println!("Adding {num}, repeated section {first:?}");
                    continue 'a;
                }
            }
        }
    }
    println!("Final sum: {sum}")
}
