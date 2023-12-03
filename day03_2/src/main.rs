use std::io::{self, BufRead};

fn main() {
    let mut lock = io::stdin().lock();
    let mut buf = String::new();
    let mut sum: u32 = 0;
    let mut symbols = vec![Vec::<usize>::new(), Vec::<usize>::new()];
    let mut numbers = vec![
        Vec::<(u32, usize, usize)>::new(),
        Vec::<(u32, usize, usize)>::new(),
        Vec::<(u32, usize, usize)>::new(),
    ];
    let mut line_number: usize = 0;
    while let Ok(read) = lock.read_line(&mut buf) {
        if read == 0 {
            break;
        }
        line_number += 1;
        symbols[line_number % 2] = vec![];
        numbers[line_number % 3] = vec![];
        let line = buf.as_bytes();
        let mut i: usize = 0;
        while i < line.len() - 1 {
            if line[i].is_ascii_digit() {
                let mut tuple: (u32, usize, usize) = (
                    (line[i] - '0' as u8) as u32,
                    if i > 0 { i - 1 } else { i },
                    i + 1,
                );
                while i + 1 < line.len() - 1 && line[i + 1].is_ascii_digit() {
                    i += 1;
                    tuple.0 = tuple.0 * 10 + (line[i] - '0' as u8) as u32;
                    tuple.2 += 1;
                }
                numbers[line_number % 3].push(tuple);
            } else if line[i] == '*' as u8 {
                symbols[line_number % 2].push(i);
            }
            i += 1;
        }
        for i in &symbols[(line_number - 1) % 2] {
            let mut gear_ratio: u32 = 1;
            let mut gear_amount: u32 = 0;
            for j in 0..3 {
                for t in &numbers[j] {
                    if i >= &t.1 && i <= &t.2 {
                        gear_ratio *= t.0;
                        gear_amount += 1;
                    }
                }
                numbers[j].retain(|t| i < &t.1 || i > &t.2)
            }
            if gear_amount == 2 {
                sum += gear_ratio;
            }
        }
        buf.clear();
    }
    for i in &symbols[line_number % 2] {
        let mut gear_ratio: u32 = 1;
        let mut gear_amount: u32 = 0;
        for j in [line_number % 3, (line_number - 1) % 3] {
            for t in &numbers[j] {
                if i >= &t.1 && i <= &t.2 {
                    gear_ratio *= t.0;
                    gear_amount += 1;
                }
            }
            numbers[j].retain(|t| i < &t.1 || i > &t.2)
        }
        if gear_amount == 2 {
            sum += gear_ratio;
        }
    }
    println!("{sum}");
}
