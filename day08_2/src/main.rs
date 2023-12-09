use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead, BufReader},
};

fn main() {
    let mut lock = BufReader::new(io::stdin().lock());
    let mut buf = String::new();

    let _ = lock.read_line(&mut buf);
    let mut dirs: Vec<bool> = Vec::new();
    for ch in buf.chars() {
        match ch {
            'L' => dirs.push(false),
            'R' => dirs.push(true),
            _ => break,
        }
    }
    let _ = lock.read_line(&mut buf);
    buf.clear();

    let mut map: HashMap<u16, (u16, u16)> = HashMap::new();
    let mut nodes: Vec<u16> = Vec::new();
    let mut curs: Vec<u16> = Vec::new();

    while let Ok(read) = lock.read_line(&mut buf) {
        if read == 0 {
            break;
        }
        let mut ch = buf.chars();
        let mut node = 0u16;
        let mut left = 0u16;
        let mut right = 0u16;

        node += ((ch.next().unwrap() as u8 - 'A' as u8) as u16) << 10;
        node += ((ch.next().unwrap() as u8 - 'A' as u8) as u16) << 5;
        node += (ch.next().unwrap() as u8 - 'A' as u8) as u16;
        ch.next();
        ch.next();
        ch.next();
        ch.next();

        left += ((ch.next().unwrap() as u8 - 'A' as u8) as u16) << 10;
        left += ((ch.next().unwrap() as u8 - 'A' as u8) as u16) << 5;
        left += (ch.next().unwrap() as u8 - 'A' as u8) as u16;
        ch.next();
        ch.next();

        right += ((ch.next().unwrap() as u8 - 'A' as u8) as u16) << 10;
        right += ((ch.next().unwrap() as u8 - 'A' as u8) as u16) << 5;
        right += (ch.next().unwrap() as u8 - 'A' as u8) as u16;

        if node << 11 == 0 {
            curs.push(node);
        }
        nodes.push(node);

        map.insert(node, (left, right));

        buf.clear();
    }

    let mut nexts: HashMap<u16, (u16, HashSet<usize>)> = HashMap::new();

    for n in nodes {
        let mut cur = n;
        let mut ends: HashSet<usize> = HashSet::new();
        for dist in 0..dirs.len() * (1 << 16) {
            if (cur << 11) >> 11 == ('Z' as u8 - 'A' as u8) as u16 {
                ends.insert(dist);
            }
            if dirs[dist % dirs.len()] {
                cur = map.get(&cur).unwrap().1;
            } else {
                cur = map.get(&cur).unwrap().0;
            }
        }
        nexts.insert(n, (cur, ends));
    }

    let mut dist = 0usize;
    loop {
        let mut ends: HashSet<usize> = nexts.get(&curs[0]).unwrap().1.clone();
        ends.retain(|x| {
            for i in 1..curs.len() {
                if !nexts.get(&curs[i]).unwrap().1.contains(x) {
                    return false;
                }
            }
            true
        });
        if !ends.is_empty() {
            dist += ends.iter().min().unwrap();
            break;
        }
        for i in 0..curs.len() {
            curs[i] = nexts.get(&curs[i]).unwrap().0;
        }
        dist += dirs.len() * (1 << 16);
    }

    println!("{:?}", dist);
}
