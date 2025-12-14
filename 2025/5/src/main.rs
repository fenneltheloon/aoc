use std::{
    fmt::{Debug, Display},
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Sub},
};

use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
enum RangeBound {
    Start,
    Stop,
}

#[derive(Clone, Debug)]
struct Ranges<T>(Vec<(T, RangeBound)>);

impl<T> Ranges<T>
where
    T: Ord + Copy + Sub<u64, Output = T>,
{
    fn new() -> Self {
        Ranges(vec![])
    }

    fn insert(&mut self, low_bound: T, up_bound: T) {
        self.0.push((low_bound, RangeBound::Start));
        self.0.push((up_bound, RangeBound::Stop));
    }

    fn simplify(&mut self) {
        if self.0.is_empty() {
            return;
        }

        assert_eq!(self.0.len() % 2, 0);
        self.0.sort_by(|a, b| a.0.cmp(&b.0));
        assert!(matches!(self.0[0].1, RangeBound::Start));
        let mut start_indices = vec![0usize];
        let mut seen_start = 1u32;
        for (i, el) in self.0.iter().enumerate().skip(1) {
            if matches!(el.1, RangeBound::Stop) {
                seen_start -= 1;
                continue;
            }
            if seen_start == 0 && self.0[i - 1].0 < el.0 - 1 {
                start_indices.push(i);
            }
            seen_start += 1;
        }
        assert!(matches!(self.0[self.0.len() - 1].1, RangeBound::Stop));
        start_indices.push(self.0.len());
        let mut final_vec = Ranges::new();
        for (a, b) in start_indices.iter().zip(start_indices.iter().skip(1)) {
            final_vec.0.push(self.0[*a]);
            final_vec.0.push(self.0[b - 1]);
        }
        *self = final_vec;
        // Also need to check for ranges one apart
    }

    fn is_in_range(&self, value: T) -> bool {
        match self.0.binary_search_by(|e| e.0.cmp(&value)) {
            Ok(_) => true,
            Err(n) => {
                if n == 0 || n >= self.0.len() {
                    return false;
                }
                match self.0[n].1 {
                    RangeBound::Start => false,
                    RangeBound::Stop => true,
                }
            }
        }
    }
}

impl<T> Display for Ranges<T>
where
    T: Sub<T, Output = T> + Display + Debug + Copy + Add<u64, Output = T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (a, b) in self.0.iter().tuples() {
            let del = b.0 - a.0 + 1;
            writeln!(f, "({a:?}, {b:?}): {del}")?;
        }
        Ok(())
    }
}

// fn is_in_range(a: &[(&(u64, RangeBound), &(u64, RangeBound))], val: u64) -> bool {
//     for t in a {
//         if t.0 .0 <= val && val <= t.1 .0 {
//             return true;
//         }
//     }
//     false
// }

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut input_lines = BufReader::new(input).lines();
    let mut test_area = Ranges::new();

    let ranges = input_lines
        .by_ref()
        .take_while(|e| e.as_ref().is_ok_and(|f| !f.is_empty()));
    for range in ranges {
        let range = range.unwrap();
        let mut range = range.split("-").map(|f| f.parse::<u64>().unwrap());
        test_area.insert(range.next().unwrap(), range.next().unwrap());
    }
    // let g = test_area.clone();
    // let mut test_test_area: Vec<(&(u64, RangeBound), &(u64, RangeBound))> =
    //     g.0.iter().tuples::<(_, _)>().collect();
    // test_test_area.sort_by(|e, f| e.0 .0.cmp(&f.0 .0));
    test_area.simplify();
    // println!("Range vector: {0:#?}", test_test_area);
    // println!("{test_test_area:#?}");
    println!("{test_area}");
    // let mut answer = 0u64;
    // let mut answer2 = 0u64;

    // for line in input_lines {
    //     let line = line.unwrap().parse::<u64>().unwrap();
    //     let is_in_1 = is_in_range(&test_test_area, line);
    //     let is_in_2 = test_area.is_in_range(line);

    //     answer += is_in_1 as u64;
    //     answer2 += is_in_2 as u64;
    //     if is_in_1 != is_in_2 {
    //         println!("Discrepancy: {line} in correct is {is_in_1} and wrong is {is_in_2}");
    //     }
    // }
    // let answer = input_lines
    //     .map(|e| e.unwrap().parse::<u64>().unwrap())
    //     .fold(0u64, |acc, e| acc + is_in_range(&test_test_area, e) as u64);

    let answer = test_area.0.iter().tuples::<(_, _)>().fold(0u64, |acc, e| {
        let del = e.1 .0 - e.0 .0 + 1;
        println!("{e:?}: {del}");
        acc + del
    });

    println!("Answer: {answer}");
}
