use std::{
    cmp::{max, min},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Copy, Clone, Debug)]
struct Point(u32, u32);

fn permute_list<T: Copy>(a: &[T]) -> Vec<(T, T)> {
    let mut r = vec![];
    for i in 0..a.len() - 1 {
        for j in i + 1..a.len() {
            r.push((a[i], a[j]));
        }
    }

    r
}

fn area(p: &(Point, Point)) -> u64 {
    ((max(p.0 .0, p.1 .0) - min(p.0 .0, p.1 .0) + 1) as u64)
        * ((max(p.0 .1, p.1 .1) - min(p.0 .1, p.1 .1) + 1) as u64)
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let lines = BufReader::new(input)
        .lines()
        .map(|e| {
            e.unwrap()
                .split(',')
                .map(|f| f.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let lines = lines.iter().map(|e| Point(e[0], e[1])).collect::<Vec<_>>();

    let perm_list = permute_list(&lines);

    let mut area_list = perm_list
        .iter()
        .map(|e| (e.0, e.1, area(e)))
        .collect::<Vec<_>>();

    area_list.sort_by_key(|e| e.2);
    area_list.reverse();

    println!("{0:?}", area_list[0]);
}
