use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Eq, PartialOrd, PartialEq, Ord, Copy, Clone, Hash, Debug)]
struct Coord(i32, i32, i32);

fn permute_list<T: Copy>(a: &[T]) -> Vec<(T, T)> {
    let mut f: Vec<(T, T)> = vec![];
    for i in 0..a.len() - 1 {
        for j in i + 1..a.len() {
            f.push((a[i], a[j]));
        }
    }
    f
}

impl Coord {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Coord(x, y, z)
    }

    fn distance(&self, a: &Coord) -> f64 {
        // println!("Coords: {self:?} {a:?}");
        let delx = (self.0 - a.0) as i64;
        let dely = (self.1 - a.1) as i64;
        let delz = (self.2 - a.2) as i64;

        f64::sqrt((delx * delx + dely * dely + delz * delz) as f64)
    }
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let lines: Vec<Coord> = BufReader::new(input)
        .lines()
        .map(|e| {
            e.unwrap()
                .split(',')
                .map(|f| f.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|e| Coord::new(e[0], e[1], e[2]))
        .collect();

    let mut graph: HashMap<Coord, HashSet<Coord>> = HashMap::with_capacity(lines.len());

    let mut dist_pair_list = permute_list(&lines)
        .iter()
        .map(|e| {
            let dist = e.0.distance(&e.1);
            (e.0, e.1, dist)
        })
        .collect::<Vec<_>>();

    dist_pair_list.sort_by(|e, f| e.2.partial_cmp(&f.2).unwrap());

    for (i, dist) in dist_pair_list.iter().take(1000).enumerate() {
        print!("{dist:?}\t");
        if i % 3 == 0 {
            println!();
        }
    }

    let dist_pair_list = dist_pair_list
        .iter()
        .map(|e| (e.0, e.1))
        .collect::<Vec<_>>();

    for pair in dist_pair_list.clone().iter().take(1000) {
        match graph.get_mut(&pair.0) {
            Some(l) => {
                l.insert(pair.1);
            }
            None => {
                graph.insert(pair.0, HashSet::from([pair.1]));
            }
        }
        match graph.get_mut(&pair.1) {
            Some(l) => {
                l.insert(pair.0);
            }
            None => {
                graph.insert(pair.1, HashSet::from([pair.0]));
            }
        }
    }

    // s is the working stack
    let mut s: HashSet<Coord> = HashSet::new();
    // Each entry in this will be a list corresponding to a network
    let mut networks: Vec<HashSet<Coord>> = vec![HashSet::new()];
    let mut networks_len = 1usize;

    while !graph.is_empty() {
        let curr = if s.is_empty() {
            networks.push(HashSet::new());
            networks_len += 1;
            println!("Current graph len: {0}", graph.len());
            *graph.keys().next().unwrap()
        } else {
            // println!("{s:?}");
            let temp = *s.iter().next().unwrap();
            s.remove(&temp);
            temp
        };

        // println!("curr: {curr:?}");

        let node = match graph.remove_entry(&curr) {
            Some(n) => n,
            None => continue,
        };

        // println!("node: {node:?}");

        networks[networks_len - 1].insert(node.0);
        s.extend(node.1.iter());
    }

    networks.sort_by_key(|e| e.len());

    for network in networks.iter() {
        println!("{network:?}");
        println!("Size: {0}", network.len());
    }

    let answer = networks
        .iter()
        .rev()
        .take(3)
        .map(|e| e.len())
        .fold(1usize, usize::strict_mul);

    println!("Answer: {answer}");
}
