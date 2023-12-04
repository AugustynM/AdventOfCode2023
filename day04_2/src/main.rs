use std::io::{self, BufRead};

fn main() {
    let mut lock = io::stdin().lock();
    let mut buf = String::new();
    let mut sum: u32 = 0;
    let mut line_number: usize = 0;
    let mut cards: Vec<u32> = vec![1];
    while let Ok(read) = lock.read_line(&mut buf) {
        if read == 0 {
            break;
        }
        if cards.len() <= line_number {
            cards.push(1);
        }
        sum += cards[line_number];

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
        let mut val: usize = 0;
        for a in &winning_numbers {
            for b in &my_numbers {
                if a == b {
                    val += 1;
                }
            }
        }
        if val > 0 {
            while cards.len() <= line_number + val {
                cards.push(1);
            }
            for i in line_number + 1..=line_number + val {
                cards[i] += cards[line_number];
            }
        }
        buf.clear();
        line_number += 1;
    }
    println!("{sum}");
}
