use std::io::{self, BufRead};

fn main() {
    let mut lock = io::stdin().lock();
    let mut buf = String::new();
    let mut sum: u32 = 0;
    while let Ok(read) = lock.read_line(&mut buf) {
        if read == 0 {
            break;
        }
        let mut parts = buf.split(':').last().expect("Error").split('|');
        let mut winning_numbers: Vec<u32> = vec![];
        let mut my_numbers: Vec<u32> = vec![];
        for x in parts.next().expect("Error").split(' ') {
            let y = x.trim();
            if y.len() > 0 {
                winning_numbers.push(y.parse::<u32>().expect("Error"));
            }
        }
        for x in parts.next().expect("Error").split(' ') {
            let y = x.trim();
            if y.len() > 0 {
                my_numbers.push(y.parse::<u32>().expect("Error"));
            }
        }
        let mut val: u32 = 0;
        for a in &winning_numbers {
            for b in &my_numbers {
                if a == b {
                    if val == 0 {
                        val = 1;
                    } else {
                        val <<= 1;
                    }
                }
            }
        }
        sum += val;
        buf.clear();
    }
    println!("{sum}");
}
