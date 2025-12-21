use std::{
    cmp::max,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Copy, Clone, Debug)]
struct Point(u32, u32);

#[derive(Debug, Copy, Clone)]
struct Distances(f64, f64, f64, f64);

impl Point {
    // TODO: need to make max_x and max_y not the same as some of the points, hypotenuse is off because of it?
    fn distances(&self, max_x: u32, max_y: u32) -> Distances {
        let delx = max_x - self.0;
        let dely = max_y - self.1;

        let distance1 = f64::sqrt(self.0 as f64 * self.0 as f64 + dely as f64 * dely as f64);
        let distance2 = f64::sqrt(delx as f64 * delx as f64 + self.1 as f64 * self.1 as f64);
        let distance3 = f64::sqrt(delx as f64 * delx as f64 + dely as f64 * dely as f64);
        let distance4 = f64::sqrt(self.0 as f64 * self.0 as f64 + self.1 as f64 * self.1 as f64);

        Distances(distance1, distance2, distance3, distance4)
    }
}

fn print_list(a: &[(Point, Distances)]) {
    for item in a {
        println!("{0:?} {1:?}", item.0, item.1);
    }
    println!()
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let lines = lines
        .map(|e| {
            let g = e
                .unwrap()
                .split(',')
                .map(|f| f.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            Point(g[0], g[1])
        })
        .collect::<Vec<_>>();

    let (max_x, max_y) = lines
        .iter()
        .fold((0, 0), |acc, e| (max(acc.0, e.0), max(acc.1, e.1)));

    println!("Max X: {max_x} May Y: {max_y}");

    let mut points = lines
        .iter()
        .map(|e| (*e, e.distances(max_x, max_y)))
        .collect::<Vec<_>>();

    points.sort_by(|e, f| e.1 .0.partial_cmp(&f.1 .0).unwrap());
    let point1 = points[0];
    print_list(&points[0..5]);
    points.sort_by(|e, f| e.1 .1.partial_cmp(&f.1 .1).unwrap());
    let point2 = points[0];
    print_list(&points[0..5]);
    points.sort_by(|e, f| e.1 .2.partial_cmp(&f.1 .2).unwrap());
    let point3 = points[0];
    print_list(&points[0..5]);
    points.sort_by(|e, f| e.1 .3.partial_cmp(&f.1 .3).unwrap());
    let point4 = points[0];
    print_list(&points[0..5]);

    // Decide which pair of corners to use
    let pair1 = point1.1 .0 * point2.1 .1;
    let pair2 = point3.1 .2 * point4.1 .3;

    let answer = if pair1 < pair2 {
        println!("{point1:?} {point2:?}");
        (point2.0 .0 - point1.0 .0 + 1) as u64 * (point1.0 .1 - point2.0 .1 + 1) as u64
    } else {
        println!("{point3:?} {point4:?}");
        (point3.0 .0 - point4.0 .0 + 1) as u64 * (point3.0 .1 - point4.0 .1 + 1) as u64
    };

    println!("Answer: {answer}");
}
