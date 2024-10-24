use std::collections::HashMap;
use std::collections::VecDeque;
use std::io;

enum Operation {
    Direct,
    Not,
    And,
    Or,
    Lshift,
    Rshift,
}

struct Assignment {
    op: Operation,
    in1: String,
    in2: Option<String>,
    out: String,
}

fn main() {
    let mut wires: HashMap<String, u16> = HashMap::with_capacity(339);
    // Pop front, push back
    let mut assignments: VecDeque<Assignment> = VecDeque::with_capacity(339);

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let line: Vec<&str> = line.split(" ").collect();

        // 0 -> a
        // a -> b
        if line.len() == 3 {
            assignments.push_back(Assignment {
                op: Operation::Direct,
                in1: line[0].to_string(),
                in2: None,
                out: line[2].to_string(),
            })
        // NOT a -> b
        } else if line.len() == 4 {
            assignments.push_back(Assignment {
                op: Operation::Not,
                in1: line[1].to_string(),
                in2: None,
                out: line[3].to_string(),
            })
        // a AND b -> c
        // a OR b -> c
        // a LSHIFT b -> c
        // a RSHIFT b -> c
        } else {
            match line[1] {
                "AND" => assignments.push_back(Assignment {
                    op: Operation::And,
                    in1: line[0].to_string(),
                    in2: Some(line[2].to_string()),
                    out: line[4].to_string(),
                }),
                "OR" => assignments.push_back(Assignment {
                    op: Operation::Or,
                    in1: line[0].to_string(),
                    in2: Some(line[2].to_string()),
                    out: line[4].to_string(),
                }),
                "LSHIFT" => assignments.push_back(Assignment {
                    op: Operation::Lshift,
                    in1: line[0].to_string(),
                    in2: Some(line[2].to_string()),
                    out: line[4].to_string(),
                }),
                "RSHIFT" => assignments.push_back(Assignment {
                    op: Operation::Rshift,
                    in1: line[0].to_string(),
                    in2: Some(line[2].to_string()),
                    out: line[4].to_string(),
                }),
                _ => panic!("Bad parse"),
            }
        }
    }

    // Seed round
    let mut al = assignments.len();
    println!("{al}");
    for _ in 0..al {
        let a = assignments.pop_front().unwrap();
        if matches!(a.op, Operation::Direct) {
            if let Ok(n) = a.in1.parse::<u16>() {
                println!("{} -> {}", n, a.out);
                wires.insert(a.out, n);
                al -= 1;
                continue;
            }
        }
        assignments.push_back(a);
    }

    while !assignments.is_empty() {
        let a = assignments.pop_front().unwrap();
        let n1 = match wires.get(&a.in1) {
            None => match a.in1.parse::<u16>() {
                Ok(n) => n,
                Err(_) => {
                    assignments.push_back(a);
                    continue;
                }
            },
            Some(n) => *n,
        };
        if a.in2.is_none() {
            match a.op {
                Operation::Direct => {
                    println!("{} -> {}", n1, a.out);
                    wires.insert(a.out, n1);
                    continue;
                }
                Operation::Not => {
                    let new = !n1;
                    println!("{new} -> {}", a.out);
                    wires.insert(a.out, new);
                    continue;
                }
                _ => panic!("One input given to operation that requires two inputs."),
            }
        }
        let n2 = a.in2.as_ref().unwrap();
        let n2 = match wires.get(n2) {
            None => match n2.parse::<u16>() {
                Ok(n) => n,
                Err(_) => {
                    assignments.push_back(a);
                    continue;
                }
            },
            Some(n) => *n,
        };
        match a.op {
            Operation::And => {
                let new = n1 & n2;
                println!("{new} -> {}", a.out);
                wires.insert(a.out, new);
            }
            Operation::Or => {
                let new = n1 | n2;
                println!("{new} -> {}", a.out);
                wires.insert(a.out, new);
            }
            Operation::Lshift => {
                let new = n1 << n2;
                println!("{new} -> {}", a.out);
                wires.insert(a.out, new);
            }
            Operation::Rshift => {
                let new = n1 >> n2;
                println!("{new} -> {}", a.out);
                wires.insert(a.out, new);
            }
            _ => panic!("How did we get here?"),
        };
    }
    println!("{}", wires.len());
    println!("a: {}", wires.get("a").unwrap());
}
