use std::io::{self, BufRead, BufReader};

fn main() {
    let mut lock = BufReader::new(io::stdin().lock());
    let mut buf = String::new();
    let mut sum = 0u32;
    let mut points = Vec::<(u32, u32)>::new();

    let mut y = 0u32;
    let mut maxx = 0u32;
    let maxy;

    while let Ok(read) = lock.read_line(&mut buf) {
        if read == 0 {
            break;
        }
        let mut x = 0u32;
        for ch in buf.chars() {
            if ch == '#' {
                points.push((x, y));
            }
            x += 1;
            if x > maxx {
                maxx = x;
            }
        }
        y += 1;
        buf.clear();
    }

    maxy = y;
    maxx+=1;
    let mut i = 0;
    while i <  maxx {
        if !points.iter().any(|a| a.0 == i) {
            for a in &mut points {
                if a.0 > i {
                    a.0 += 1;
                }
            }
            i+=1;
        }
        i+=1;
    }
    
    let mut i = 0;
    while i < maxy {
        if !points.iter().any(|a| a.1 == i) {
            for a in &mut points {
                if a.1 > i {
                    a.1 += 1;
                }
            }
            i+=1;
        }
        i+=1;
    }

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            sum += points[i].0.abs_diff(points[j].0) + points[i].1.abs_diff(points[j].1);
        }
    }

    println!("{sum}");
}
