use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader},
};

fn main() {
    let mut lock = BufReader::new(io::stdin().lock());
    let mut buf = String::new();
    let mut input: Vec<(u32, u32)> = Vec::new();
    let map: HashMap<char, u32> = HashMap::from([
        ('2', 0),
        ('3', 1),
        ('4', 2),
        ('5', 3),
        ('6', 4),
        ('7', 5),
        ('8', 6),
        ('9', 7),
        ('T', 8),
        ('J', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]);
    while let Ok(read) = lock.read_line(&mut buf) {
        if read == 0 {
            break;
        }
        let mut parts = buf.split(' ');

        let mut cnt = vec![0; 13];
        let mut h: u32 = 0;
        let mut i: u32 = 0;
        for ch in parts.next().unwrap().chars() {
            let c = map.get(&ch).unwrap();
            cnt[*c as usize] += 1;
            h += c << 16 >> (i * 4);
            i += 1;
        }

        if cnt.contains(&5) {
            h += 6 << 20;
        } else if cnt.contains(&4) {
            h += 5 << 20;
        } else if cnt.contains(&3) && cnt.contains(&2) {
            h += 4 << 20;
        } else if cnt.contains(&3) {
            h += 3 << 20;
        } else if cnt.iter().filter(|a| **a == 2).count() == 2 {
            h += 2 << 20;
        } else if cnt.contains(&2) {
            h += 1 << 20;
        }

        let v: u32 = parts.next().unwrap().trim().parse::<u32>().unwrap();
        input.push((h, v));
        buf.clear();
    }
    input.sort();
    let mut sum: u32 = 0;
    for i in 0..input.len() {
        sum += (i as u32 + 1) * input[i].1;
    }
    println!("{sum}");
}
