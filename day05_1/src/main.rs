use std::io::{self, BufRead, BufReader};

fn main() {
    let mut lock = BufReader::new(io::stdin().lock());
    let mut buf = String::new();
    let mut seeds: Vec<u64> = vec![];
    let mut maps: Vec<Vec<(u64, u64, u64)>> = vec![vec![]; 7];
    let _ = lock.read_line(&mut buf);
    let mut parts = buf.split(' ');
    parts.next();
    while let Some(s) = parts.next() {
        if let Ok(k) = s.trim().parse::<u64>() {
            seeds.push(k);
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

    let min = seeds
        .into_iter()
        .map(|mut val| {
            for i in 0..7usize {
                for (b, a, r) in &maps[i] {
                    if val >= *a && val < a + r {
                        val = val + b - a;
                        break;
                    }
                }
            }
            val
        })
        .min()
        .unwrap();
    println!("{min}");
}
