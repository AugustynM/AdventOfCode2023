use std::{
    cmp::{max, min},
    io::{self, BufRead, BufReader},
};

fn main() {
    let mut lock = BufReader::new(io::stdin().lock());
    let mut buf = String::new();
    let mut segments: Vec<(u64, u64)> = vec![];
    let mut maps: Vec<Vec<(u64, u64, u64)>> = vec![vec![]; 7];
    let _ = lock.read_line(&mut buf);
    let mut parts = buf.split(' ');
    parts.next();
    while let Some(s) = parts.next() {
        if let Ok(k) = s.trim().parse::<u64>() {
            segments.push((k, parts.next().unwrap().trim().parse::<u64>().unwrap()));
        }
    }
    let _ = lock.read_line(&mut buf);
    buf.clear();

    for i in 0..7usize {
        let _ = lock.read_line(&mut buf);
        buf.clear();
        while let Ok(read) = lock.read_line(&mut buf) {
            if read < 2 {
                break;
            }
            let mut parts = buf.split(' ');
            maps[i].push((
                parts.next().unwrap().trim().parse::<u64>().unwrap(),
                parts.next().unwrap().trim().parse::<u64>().unwrap(),
                parts.next().unwrap().trim().parse::<u64>().unwrap(),
            ));
            buf.clear();
        }
    }

    for i in 0..7usize {
        let mut segments2: Vec<(u64, u64)> = vec![];
        for (b, a, r) in &maps[i] {
            let mut segments3: Vec<(u64, u64)> = vec![];
            while let Some((st, ra)) = segments.pop() {
                if st + ra < *a || st > a + r {
                    segments3.push((st, ra));
                } else {
                    if st < *a {
                        segments3.push((st, a - st));
                    }
                    if st + ra > a + r {
                        segments3.push((a + r, st + ra - (a + r)));
                    }
                    segments2.push((max(st, *a) + b - a, min(st + ra, a + r) - max(st, *a)));
                }
            }
            segments = segments3.clone();
        }
        for i in segments {
            segments2.push(i);
        }
        segments = segments2.clone();
    }

    let min = segments.into_iter().min().unwrap().0;
    println!("{min}");
}
