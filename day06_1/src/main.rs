use std::io::{self, BufRead, BufReader};

fn main() {
    let mut lock = BufReader::new(io::stdin().lock());
    let mut buf = String::new();
    let _ = lock.read_line(&mut buf);
    let mut inputs: Vec<(u32, u32)> = vec![];
    let mut output: u32 = 1;
    let parts = buf.split(' ');
    for a in parts {
        if let Ok(n) = a.trim().parse::<u32>() {
            inputs.push((n, 0));
        }
    }
    let mut i: usize = 0;
    buf.clear();
    let _ = lock.read_line(&mut buf);
    let parts = buf.split(' ');
    for a in parts {
        if let Ok(n) = a.trim().parse::<u32>() {
            inputs[i].1 = n;
            i += 1;
        }
    }
    for (time, distance) in inputs {
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
    }
    println!("{output}");
}
