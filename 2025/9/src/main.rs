use std::{
    cmp::{max, min},
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    sync::{Arc, Mutex, RwLock},
    thread::{self, JoinHandle},
};

#[derive(Copy, Clone, Debug)]
struct Point(usize, usize);

#[derive(Copy, Clone, Debug)]
struct Rect(Point, Point, usize);

#[derive(Clone, Copy)]
enum Tile {
    Red,
    Green,
    Inside,
    Outside,
}

impl Rect {
    fn contains(&self, p: &Point) -> bool {
        // Check x and y
        let minx = min(self.0 .0, self.1 .0);
        let miny = min(self.0 .1, self.1 .1);
        let maxx = max(self.0 .0, self.1 .0);
        let maxy = max(self.0 .1, self.1 .1);

        minx <= p.0 && p.0 <= maxx && miny <= p.1 && p.1 <= maxy
    }
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Red => '#',
            Tile::Green => 'X',
            Tile::Inside => ',',
            Tile::Outside => '.',
        }
    }
}

impl From<Tile> for &str {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Red => "#",
            Tile::Green => "X",
            Tile::Inside => ",",
            Tile::Outside => ".",
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Red),
            'X' => Ok(Tile::Green),
            ',' => Ok(Tile::Inside),
            '.' => Ok(Tile::Outside),
            e => Err(format!("Unexpected character {e}")),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&char::from(*self).to_string())?;
        Ok(())
    }
}

fn permute_list<T: Copy>(a: &[T]) -> Vec<(T, T)> {
    let mut r = vec![];
    for i in 0..a.len() - 1 {
        for j in i + 1..a.len() {
            r.push((a[i], a[j]));
        }
    }
    r
}

fn area(p: &(Point, Point)) -> usize {
    (max(p.0 .0, p.1 .0) - min(p.0 .0, p.1 .0) + 1)
        * (max(p.0 .1, p.1 .1) - min(p.0 .1, p.1 .1) + 1)
}

// fn is_good_rect(rect: (Point, Point, usize), field: &[Vec<Tile>]) -> bool {
//     println!("Candidate {rect:?}");
//     let upper_x = max(rect.0 .0, rect.1 .0);
//     let lower_x = min(rect.0 .0, rect.1 .0);
//     let upper_y = max(rect.0 .1, rect.1 .1);
//     let lower_y = min(rect.0 .1, rect.1 .1);

//     for row in field.iter().take(upper_x + 1).skip(lower_x) {
//         for tile in row.iter().take(upper_y + 1).skip(lower_y) {
//             if matches!(tile, Tile::Outside) {
//                 println!("failed");
//                 return false;
//             }
//         }
//     }

//     println!("{rect:?} passed");
//     true
// }

// fn find_rect(
//     perm_list: &[(Point, Point, usize)],
//     up_bound: usize,
//     low_bound: usize,
//     field: &Vec<Vec<Tile>>,
// ) -> (Point, Point, usize) {
//     if up_bound - low_bound == 0 {
//         if is_good_rect(perm_list[low_bound], field) {
//             return perm_list[low_bound];
//         } else {
//             return find_rect(perm_list, up_bound + 1, low_bound + 1, field);
//         }
//     }
//     let curr_rect_index = (up_bound - low_bound) / 2 + low_bound;
//     let curr_rect = perm_list[curr_rect_index];
//     if is_good_rect(curr_rect, field) {
//         // search the lower indexes
//         find_rect(perm_list, curr_rect_index, low_bound, field)
//     } else {
//         find_rect(perm_list, up_bound, curr_rect_index + 1, field)
//     }
// }

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut lines = BufReader::new(input)
        .lines()
        .map(|e| {
            e.unwrap()
                .split(',')
                .map(|f| f.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|e| Point(e[0], e[1]))
        .collect::<Vec<_>>();
    let (max_x, max_y) = lines
        .iter()
        .fold((0, 0), |acc, e| (max(acc.0, e.0), max(acc.1, e.1)));

    let field_file_path = Path::new("field_file.txt");
    let mut field: Vec<Vec<Tile>> = if field_file_path.exists() {
        vec![]
    } else {
        vec![vec![Tile::Inside; max_y + 1]; max_x + 1]
    };

    if field_file_path.exists() {
        let field_file = File::open(field_file_path).unwrap();
        let field_file_reader = BufReader::new(field_file).lines();

        for line in field_file_reader {
            let line = line.unwrap();
            let line = line.chars();
            let mut curr_row: Vec<Tile> = vec![];
            for c in line {
                curr_row.push(c.try_into().unwrap());
            }
            field.push(curr_row);
        }
    } else {
        lines.push(lines[0]);
        // Outlining the shape in the grid
        for (point1, point2) in lines.iter().zip(lines.iter().skip(1)) {
            field[point1.0][point1.1] = Tile::Red;
            field[point2.0][point2.1] = Tile::Red;

            // x equal, change in y
            if point1.0 == point2.0 {
                let lower_y = min(point1.1, point2.1);
                let upper_y = max(point1.1, point2.1);
                field[point1.0]
                    .iter_mut()
                    .take(upper_y)
                    .skip(lower_y + 1)
                    .for_each(|e| *e = Tile::Green);
            } else {
                // y equal, change in x
                let lower_x = min(point1.0, point2.0);
                let upper_x = max(point1.0, point2.0);
                field
                    .iter_mut()
                    .take(upper_x)
                    .skip(lower_x + 1)
                    .for_each(|e| (*e)[point1.1] = Tile::Green);
            }
        }

        lines.pop();

        // Flood fill
        let mut s = vec![];
        for (i, row) in field.iter().enumerate() {
            if i == 0 || i == field.len() - 1 {
                for (j, cell) in row.iter().enumerate() {
                    if matches!(cell, Tile::Inside) {
                        s.push((i, j));
                    }
                }
            } else {
                if matches!(row[0], Tile::Inside) {
                    s.push((i, 0));
                }
                if matches!(row[row.len() - 1], Tile::Inside) {
                    s.push((i, row.len() - 1));
                }
            }
        }
        // assert!(matches!(field[max_x - 1][max_y - 1], Tile::Outside));
        // s.push((max_x - 1, max_y - 1));

        while let Some(curr) = s.pop() {
            if s.len() % 1000000 == 0 {
                println!("{}", s.len() / 1000000);
            }
            if matches!(field[curr.0][curr.1], Tile::Inside) {
                field[curr.0][curr.1] = Tile::Outside;
                // Push all adjacents
                if curr.0 > 0 && matches!(field[curr.0 - 1][curr.1], Tile::Inside) {
                    s.push((curr.0 - 1, curr.1));
                }

                if curr.0 < field.len() - 1 && matches!(field[curr.0 + 1][curr.1], Tile::Inside) {
                    s.push((curr.0 + 1, curr.1));
                }

                if curr.1 > 0 && matches!(field[curr.0][curr.1 - 1], Tile::Inside) {
                    s.push((curr.0, curr.1 - 1));
                }

                if curr.1 < field[0].len() - 1 && matches!(field[curr.0][curr.1 + 1], Tile::Inside)
                {
                    s.push((curr.0, curr.1 + 1));
                }
            }
        }

        // Write to field file
        // let field_file = File::create(field_file_path).unwrap();
        // let mut ffw = BufWriter::new(field_file);

        // for row in field.iter() {
        //     for item in row {
        //         write!(ffw, "{}", item).unwrap();
        //     }
        //     writeln!(ffw).unwrap();
        // }
    }

    let perm_list = permute_list(&lines);

    let mut area_list = perm_list
        .iter()
        .map(|e| Rect(e.0, e.1, area(e)))
        .collect::<Vec<_>>();

    area_list.sort_by_key(|e| e.2);
    area_list.reverse();
    let lis_len = area_list.len();
    let task_list = Arc::new(Mutex::new(area_list));
    let field = Arc::new(field);

    let known_outsides: Arc<RwLock<Vec<Point>>> = Arc::new(RwLock::new(Vec::new()));

    let num_cpus = num_cpus::get();
    let mut thread_pool: Vec<JoinHandle<_>> = Vec::with_capacity(num_cpus);
    let is_done = Arc::new(RwLock::new(false));

    for i in 0..num_cpus {
        let fd = Arc::clone(&field);
        let ko = Arc::clone(&known_outsides);
        let tl = Arc::clone(&task_list);
        let id = Arc::clone(&is_done);
        thread_pool.push(thread::spawn(move || 'a: loop {
            let is_done_lock = id.read().unwrap();
            if *is_done_lock {
                break;
            }
            drop(is_done_lock);

            let mut tl_lock = tl.lock().unwrap();
            if tl_lock.is_empty() {
                break;
            }
            let rect = tl_lock.pop().unwrap();
            drop(tl_lock);

            println!("{} / {lis_len}: {rect:?}", i + 1);
            let minx = min(rect.0 .0, rect.1 .0);
            let miny = min(rect.0 .1, rect.1 .1);
            let maxx = max(rect.0 .0, rect.1 .0);
            let maxy = max(rect.0 .1, rect.1 .1);
            let kol = ko.read().unwrap();
            for item in kol.iter() {
                if rect.contains(item) {
                    println!("failed");
                    continue 'a;
                }
            }
            drop(kol);

            for (i, row) in fd.iter().enumerate().take(maxx).skip(minx) {
                for (j, tile) in row.iter().enumerate().take(maxy).skip(miny) {
                    if matches!(tile, Tile::Outside) {
                        let mut kol = ko.write().unwrap();
                        kol.push(Point(i, j));
                        println!("failed");
                        continue 'a;
                    }
                }
            }
            println!("{rect:?} passed");
            let mut is_done_lock = id.write().unwrap();
            *is_done_lock = true;
            break 'a;
        }));
    }

    for item in thread_pool {
        item.join().unwrap();
    }
}
