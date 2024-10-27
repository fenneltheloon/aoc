use std::io;

fn main() {
    let mut tch = 0;
    let mut sch = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        tch += line.len();
        let mut line_i = line.chars();
        let mut c = line_i.next();
        while !matches!(c, None) {
            sch += 1;
            match c {
                Some('\\') => sch += 1,
                Some('"') => sch += 1,
                _ => (),
            }
            c = line_i.next();
        }
        sch += 2;
    }
    let dif = sch - tch;
    println!("{sch} - {tch} = {dif}");
}
