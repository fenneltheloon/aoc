use core::panic;
use std::io;

#[derive(Debug)]
struct Wire {
    name: String,
    value: u16,
}

fn is_num(s: &str) -> bool {
    let c = s.chars();
    for ch in c {
        if !ch.is_ascii_digit() {
            return false;
        }
    }
    return true;
}

fn is_wire(s: &str) -> bool {
    let c = s.chars();
    for ch in c {
        if !ch.is_ascii_lowercase() {
            return false;
        }
    }
    return true;
}

fn wire_get(s: &str, d: &Vec<Wire>) -> Option<u16> {
    for r in d {
        if r.name == s {
            return Some(r.value);
        }
    }
    return None;
}

fn wire_set(s: &str, d: &mut Vec<Wire>, n: u16) {
    for w in &mut *d {
        if w.name == s {
            w.value = n;
            return;
        }
    }
    d.push(Wire {
        name: s.to_string(),
        value: n,
    });
}

fn evaluate(t: &[&str], w: &mut Vec<Wire>) -> u16 {
    if t.len() == 1 {
        if is_num(t[0]) {
            return t[0].parse::<u16>().unwrap();
        }
        if is_wire(t[0]) {
            match wire_get(t[0], w) {
                Some(n) => return n,
                None => {
                    w.push(Wire {
                        name: t[0].to_string(),
                        value: 0,
                    });
                    return 0;
                }
            }
        }
        panic!("Unexpected token found. Exiting...");
    }
    if let Some(i) = t.iter().position(|n| n == &"->") {
        let e = evaluate(&t[..i], w);
        wire_set(t[i + 1], w, e);
        return e;
    }
    if let Some(i) = t.iter().position(|n| n == &"AND") {
        let lh = evaluate(&t[..i], w);
        let rh = evaluate(&t[i + 1..], w);
        return lh & rh;
    }
    if let Some(i) = t.iter().position(|n| n == &"OR") {
        let lh = evaluate(&t[..i], w);
        let rh = evaluate(&t[i + 1..], w);
        return lh | rh;
    }
    if let Some(i) = t.iter().position(|n| n == &"LSHIFT") {
        let lh = evaluate(&t[..i], w);
        let rh = evaluate(&t[i + 1..], w);
        return lh << rh;
    }
    if let Some(i) = t.iter().position(|n| n == &"RSHIFT") {
        let lh = evaluate(&t[..i], w);
        let rh = evaluate(&t[i + 1..], w);
        return lh >> rh;
    }
    if let Some(i) = t.iter().position(|n| n == &"NOT") {
        let e = evaluate(&t[i + 1..], w);
        return !e;
    }
    panic!("Expression does not parse correctly. Exiting...");
}

fn main() {
    let mut wires: Vec<Wire> = Vec::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let tokens_str: Vec<&str> = line.split(" ").collect();
        let _ = evaluate(&tokens_str, &mut wires);
    }
    println!("{:#?}", wires);
    println!("Wire a: {}", wire_get("a", &wires).unwrap());
}
