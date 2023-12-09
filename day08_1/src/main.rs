use std::{
    collections::HashMap,
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

    let mut nodes: HashMap<u16, (u16, u16)> = HashMap::new();

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

        nodes.insert(node, (left, right));

        buf.clear();
    }

    let end = ((('Z' as u8 - 'A' as u8) as u16) << 10)
        + ((('Z' as u8 - 'A' as u8) as u16) << 5)
        + (('Z' as u8 - 'A' as u8) as u16);
    let mut dist = 0usize;
    let mut cur = 0;
    while cur != end {
        if dirs[dist % dirs.len()] {
            cur = nodes.get(&cur).unwrap().1;
        } else {
            cur = nodes.get(&cur).unwrap().0;
        }
        dist += 1;
    }

    println!("{dist}");
}
