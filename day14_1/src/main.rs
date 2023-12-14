use std::io::{self, BufRead, BufReader};

#[derive(PartialEq, Eq)]
enum Field {
    Empty,
    Round,
    Cube,
}

fn main() {
    let mut lock = BufReader::new(io::stdin().lock());
    let mut buf = String::new();
    let mut sum = 0u32;

    let mut map: Vec<Vec<Field>> = vec![];
    while let Ok(read) = lock.read_line(&mut buf) {
        if read == 0 {
            break;
        }
        let mut line: Vec<Field> = vec![];
        for ch in buf.trim_end().chars() {
            match ch {
                '.' => line.push(Field::Empty),
                '#' => line.push(Field::Cube),
                'O' => line.push(Field::Round),
                _ => panic!(),
            }
        }
        map.push(line);
        buf.clear();
    }

    for x in 0..map[0].len() {
        let mut w = map.len() as u32;
        for y in 0..map.len() {
            match map[y][x] {
                Field::Empty => {}
                Field::Round => {
                    sum += w;
                    w -= 1;
                }
                Field::Cube => w = (map.len() - y) as u32 - 1,
            }
        }
    }

    println!("{sum}");
}
