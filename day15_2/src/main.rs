use std::io::{self, BufRead, BufReader};

fn main() {
    let mut lock = BufReader::new(io::stdin().lock());
    let mut buf = String::new();
    let mut sum = 0u32;

    let mut boxes: Vec<Vec<(Vec<u8>, u8)>> = vec![Vec::new(); 256];

    let _ = lock.read_line(&mut buf);
    let mut hash: u8 = 0;
    let mut name = Vec::<u8>::new();
    let mut chars = buf.trim_end().chars();
    while let Some(c) = chars.next() {
        match c {
            ',' => {
                name = vec![];
                hash = 0;
            }
            '-' => {
                boxes[hash as usize].retain(|p| p.0 != name);
            }
            '=' => {
                let val: u8 = chars.next().unwrap().to_digit(10).unwrap() as u8;
                if let Some(a) = boxes[hash as usize].iter_mut().find(|p| p.0 == name) {
                    a.1 = val;
                } else {
                    boxes[hash as usize].push((name.clone(), val));
                }
            }
            _ => {
                name.push(c as u8);
                hash = hash.wrapping_add(c as u8);
                hash = hash.wrapping_mul(17);
            }
        }
    }

    for i in 0..boxes.len() {
        for j in 0..boxes[i].len() {
            sum += ((i + 1) * (j + 1) * (boxes[i][j].1 as usize)) as u32;
        }
    }

    println!("{sum}");
}
