use std::io::{self, BufRead, BufReader};

fn main() {
    let mut lock = BufReader::new(io::stdin().lock());
    let mut buf = String::new();
    let mut time: u64 = 0;
    let mut distance: u64 = 0;
    let mut output: u64 = 1;

    let _ = lock.read_line(&mut buf);
    for ch in buf.chars() {
        if let Some(n) = ch.to_digit(10) {
            time = 10*time + n as u64;
        }
    }
    buf.clear();

    let _ = lock.read_line(&mut buf);
    for ch in buf.chars() {
        if let Some(n) = ch.to_digit(10) {
            distance = 10*distance + n as u64;
        }
    }
    buf.clear();

    let best_time;
    if time % 2 == 1 {
        best_time = (time >> 1) + 1;
    } else {
        best_time = time >> 1;
    }
    let mut l = 0;
    let mut r = best_time;
    while l < r - 1 {
        let s = (l + r) / 2;
        if (time - s) * s > distance {
            r = s;
        } else {
            l = s;
        }
    }
    let mut out = best_time - r;
    out *= 2;
    if time % 2 == 0 {
        out += 1;
    }
    output *= out;

    println!("{output}");
}
