use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Eq, PartialOrd, PartialEq, Ord, Copy, Clone, Hash)]
struct Coord(i32, i32, i32);

fn permute_list<T: Copy>(a: &[T]) -> Vec<(T, T)> {
    let mut f: Vec<(T, T)> = vec![];
    for i in 0..a.len() - 1 {
        for j in i..a.len() {
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
        let delx = self.0 - a.0;
        let dely = self.1 - a.1;
        let delz = self.2 - a.2;

        f64::sqrt((delx * delx + dely * dely + delz * delz).into())
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

    let mut dist_pair_list: Vec<(Coord, Coord)> = permute_list(&lines);
    dist_pair_list.sort_by(|a, b| a.0.distance(&a.1).partial_cmp(&b.0.distance(&b.1)).unwrap());

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
    let mut s: Vec<Coord> = vec![];
    // Each entry in this will be a list corresponding to a network
    let mut networks: Vec<Vec<Coord>> = vec![vec![]];

    // In order to retrieve and remove 1 element from HashSet, call iter,next
    // and then clone and remove the element.
    for coord in graph.iter().clone() {}
}
