use std::io::{self, BufRead};

fn main() {
    let mut lock = io::stdin().lock();
    let mut buf = String::new();
    let mut sum: u32 = 0;
    let numbers = vec![
        vec!['-'],
        vec!['o', 'n', 'e'],
        vec!['t', 'w', 'o'],
        vec!['t', 'h', 'r', 'e', 'e'],
        vec!['f', 'o', 'u', 'r'],
        vec!['f', 'i', 'v', 'e'],
        vec!['s', 'i', 'x'],
        vec!['s', 'e', 'v', 'e', 'n'],
        vec!['e', 'i', 'g', 'h', 't'],
        vec!['n', 'i', 'n', 'e']
    ];

    while let Ok(read) = lock.read_line(&mut buf) {
        if read == 0 {
            break;
        }
        let mut progress: Vec<usize> = vec![0; 10];
        let mut first: u32 = 10;
        let mut second: u32 = 10;
        for c in buf.chars() {
            if let Some(d) = c.to_digit(10) {
                if first == 10 {
                    first = d;
                }
                second = d;
            }
            for i in 1usize..10 {
                if numbers[i][progress[i]]==c {
                    progress[i]+=1;
                    if progress[i]==numbers[i].len() {
                        if first == 10 {
                            first = i as u32;
                        }
                        second = i as u32;
                        progress[i] = 0;
                    }
                }
                else{
                    progress[i] = 0;
                    if numbers[i][progress[i]]==c {
                        progress[i]+=1;
                    }
                }
            }
        }
        if first != 10 && second != 10 {
            sum += 10 * first + second;
        } else {
            panic!();
        }
        buf.clear();
    }
    println!("{sum}");
}
