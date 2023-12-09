use std::io::{self, BufRead, BufReader};

fn main() {
    let mut lock = BufReader::new(io::stdin().lock());
    let mut buf = String::new();
    let mut sum: i32 = 0;

    while let Ok(read) = lock.read_line(&mut buf) {
        if read == 0 {
            break;
        }
        
        let mut numbers = buf
            .split(' ')
            .into_iter()
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        while numbers.iter().any(|n| *n != 0) {
            sum += numbers.last().unwrap();
            let mut t = Vec::<i32>::new();
            for i in 0..numbers.len() - 1 {
                t.push(numbers[i + 1] - numbers[i]);
            }
            numbers = t;
        }

        buf.clear();
    }

    println!("{sum}");
}
