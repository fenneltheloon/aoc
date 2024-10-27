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
            if matches!(c, Some('\\')) {
                let mut c = line_i.next().unwrap();
                match c {
                    '\\' => {}
                    '"' => {}
                    'x' => {
                        for _ in 0..2 {
                            c = line_i.next().unwrap();
                            if !c.is_ascii_hexdigit() {
                                panic!("Bad hexadecimal escape sequence.");
                            }
                        }
                    }
                    _ => panic!("Bad escape sequence"),
                };
            }
            sch += 1;
            c = line_i.next();
        }
        sch -= 2;
    }
    let dif = tch - sch;
    println!("{tch} - {sch} = {dif}");
}
