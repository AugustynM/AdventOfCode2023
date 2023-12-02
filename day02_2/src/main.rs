use std::io::{self, BufRead};

fn main() {
    let mut lock = io::stdin().lock();
    let mut buf = String::new();
    let mut sum: u32 = 0;
    while let Ok(read) = lock.read_line(&mut buf) {
        if read == 0 {
            break;
        }
        let mut min_r: u32 = 0;
        let mut min_g: u32 = 0;
        let mut min_b: u32 = 0;
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
                            if val > min_r {
                                min_r = val;
                            }
                        }
                        'g' => {
                            if val > min_g {
                                min_g = val;
                            }
                        }
                        'b' => {
                            if val > min_b {
                                min_b = val;
                            }
                        }
                        _ => panic!(),
                    }
                });
            });
        sum += min_r * min_g * min_b;
        buf.clear();
    }
    println!("{sum}");
}
