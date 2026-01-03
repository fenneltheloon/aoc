use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const BRACKETS: [char; 2] = ['[', ']'];
const PARENS: [char; 2] = ['(', ')'];

fn from_list(a: &[usize]) -> usize {
    a.iter().fold(0, |acc, e| acc ^ (1 << e))
}

fn from_str(a: &str) -> usize {
    let chars = a.trim_matches(BRACKETS).chars();
    chars.fold(0usize, |acc, e| (acc << 1) + (e == '#') as usize)
}

fn line_to_presses(line: String) -> usize {
    println!("{line}");
    let mut space_split = line.split_ascii_whitespace().collect::<Vec<_>>();
    let goal_string = space_split.remove(0);
    let mut seen = vec![];
    let mut round = 0;
    let mut working = vec![0usize];
    let goal_light = from_str(goal_string);
    let _joltages = space_split.pop().unwrap();

    let buttons = space_split
        .iter()
        .map(|e| {
            from_list(
                &e.trim_matches(PARENS)
                    .split(',')
                    .map(|f| f.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    loop {
        round += 1;
        seen.sort();
        if working.is_empty() {
            break;
        }
        println!("Round {round}");
        println!("Seen: {seen:?}");
        let mut new_stack = vec![];
        for curr in working.iter() {
            seen.push(*curr);
            for button in buttons.iter() {
                let res = curr ^ button;
                if res == goal_light {
                    println!("{round}");
                    return round;
                }
                if !seen.contains(&res) && !working.contains(&res) && !new_stack.contains(&res) {
                    new_stack.push(res);
                }
            }
        }
        working = new_stack;
    }

    panic!("Could not find a solution: {line}");
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let result = BufReader::new(input)
        .lines()
        .map(|e| e.unwrap())
        .fold(0usize, |acc, e| acc + line_to_presses(e));

    println!("Answer: {result}");
}
