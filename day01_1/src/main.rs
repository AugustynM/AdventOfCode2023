use std::io::{self, BufRead};

fn main() {
    let mut lock = io::stdin().lock();
    let mut buf = String::new();
    let mut sum: u32 = 0;
    while let Ok(read) = lock.read_line(&mut buf) {
        if read == 0 {
            break;
        }
        let mut first: u32 = 10;
        let mut second: u32 = 10;
        for c in buf.chars() {
            if let Some(d) = c.to_digit(10) {
                if first == 10 {
                    first = d;
                }
                second = d;
            }
        }
        if first != 10 && second != 10 {
            sum += 10 * first + second;
        } else {
            panic!();
        }
        buf.clear();
    }
    println!("{sum}");
}
