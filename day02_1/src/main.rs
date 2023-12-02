use std::io::{self, BufRead};

fn main() {
    let mut lock = io::stdin().lock();
    let mut buf = String::new();
    let mut sum: u32 = 0;
    let mut game_number = 0;
    while let Ok(read) = lock.read_line(&mut buf) {
        if read == 0 {
            break;
        }
        game_number += 1;
        let mut is_ok: bool = true;
        buf.split(':')
            .last()
            .expect("Error")
            .split(';')
            .for_each(|set| {
                set.split(',').for_each(|part| {
                    let mut chrs = part.chars();
                    let mut val: u32 = 0;
                    chrs.next();
                    while let Some(n) = chrs.next().expect("Error").to_digit(10) {
                        val *= 10;
                        val += n;
                    }
                    match chrs.next().expect("Error") {
                        'r' => {
                            if val > 12 {
                                is_ok = false;
                            }
                        }
                        'g' => {
                            if val > 13 {
                                is_ok = false;
                            }
                        }
                        'b' => {
                            if val > 14 {
                                is_ok = false;
                            }
                        }
                        _ => panic!(),
                    }
                });
            });
        if is_ok {
            sum += game_number;
        }
        buf.clear();
    }
    println!("{sum}");
}
