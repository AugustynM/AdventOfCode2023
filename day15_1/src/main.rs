use std::io::{self, BufRead, BufReader};

fn main() {
    let mut lock = BufReader::new(io::stdin().lock());
    let mut buf = String::new();
    let mut sum = 0u32;

    let _ = lock.read_line(&mut buf);
    let mut hash: u8 = 0;
    for ch in buf.trim_end().chars() {
        if ch == ',' {
            hash = 0;
        } else {
            hash = hash.wrapping_add(ch as u8);
            hash = hash.wrapping_mul(17);
        }
    }
    sum += hash as u32;

    println!("{sum}");
}
