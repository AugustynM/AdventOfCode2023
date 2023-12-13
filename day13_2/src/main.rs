use std::{
    cmp::min,
    io::{self, BufRead, BufReader},
};

fn check(map: &Vec<Vec<bool>>) -> u32 {
    //horizontal
    for x in 1..map.len() {
        if || -> bool {
            let mut cnt = 0;
            for a in 1..min(x + 1, map.len() - x + 1) {
                for y in 0..map[0].len() {
                    if map[x - a][y] != map[x + a - 1][y] {
                        cnt += 1;
                    }
                }
            }
            cnt == 1
        }() {
            return 100 * x as u32;
        }
    }

    //vertical
    for x in 1..map[0].len() {
        if || -> bool {
            let mut cnt = 0;
            for a in 1..min(x + 1, map[0].len() - x + 1) {
                for y in 0..map.len() {
                    if map[y][x - a] != map[y][x + a - 1] {
                        cnt += 1;
                    }
                }
            }
            cnt == 1
        }() {
            return x as u32;
        }
    }

    panic!();
}

fn main() {
    let mut lock = BufReader::new(io::stdin().lock());
    let mut buf = String::new();
    let mut sum = 0u32;
    let mut end = false;

    loop {
        let mut map: Vec<Vec<bool>> = vec![];
        while let Ok(read) = lock.read_line(&mut buf) {
            if read == 1 {
                break;
            }
            if read == 0 {
                end = true;
                break;
            }
            let mut line = Vec::<bool>::new();
            for ch in buf.trim_end().chars() {
                match ch {
                    '#' => line.push(false),
                    '.' => line.push(true),
                    _ => panic!(),
                }
            }
            map.push(line);
            buf.clear();
        }

        buf.clear();
        sum += check(&map);
        if end {
            break;
        }
    }

    println!("{sum}");
}
