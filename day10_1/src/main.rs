use std::io::{self, BufRead, BufReader};

#[derive(Clone, Copy)]
enum Pipe {
    Pipe([(i32, i32); 2]),
    Start,
    None,
}

fn main() {
    let mut lock = BufReader::new(io::stdin().lock());
    let mut buf = String::new();
    let mut len: u32 = 0;
    let mut pos = (0usize, 0usize);

    let mut map = Vec::<Vec<Pipe>>::new();
    map.push(vec![]);

    while let Ok(read) = lock.read_line(&mut buf) {
        if read == 0 {
            break;
        }

        let mut line = Vec::<Pipe>::new();
        line.push(Pipe::None);
        for ch in buf.trim().chars() {
            match ch {
                'J' => line.push(Pipe::Pipe([(-1, 0), (0, -1)])),
                'L' => line.push(Pipe::Pipe([(-1, 0), (0, 1)])),
                '|' => line.push(Pipe::Pipe([(-1, 0), (1, 0)])),
                '-' => line.push(Pipe::Pipe([(0, 1), (0, -1)])),
                '7' => line.push(Pipe::Pipe([(1, 0), (0, -1)])),
                'F' => line.push(Pipe::Pipe([(1, 0), (0, 1)])),
                'S' => {
                    pos = (map.len(), line.len());
                    line.push(Pipe::Start);
                }
                '.' => line.push(Pipe::None),
                _ => panic!(),
            }
        }
        line.push(Pipe::None);
        map.push(line);

        buf.clear();
    }

    map.push(vec![Pipe::None; map[1].len()]);
    map[0] = vec![Pipe::None; map[1].len()];

    let mut dir: (i32, i32) = (-1, 0);

    if let Pipe::Pipe(a) = map[pos.0][pos.1 + 1] {
        if a.contains(&(0, -1)) {
            dir = (0, 1);
        }
    }
    if let Pipe::Pipe(a) = map[pos.0][pos.1 - 1] {
        if a.contains(&(0, 1)) {
            dir = (0, -1);
        }
    }
    if let Pipe::Pipe(a) = map[pos.0 + 1][pos.1] {
        if a.contains(&(-1, 0)) {
            dir = (1, 0);
        }
    }

    loop {
        len += 1;
        match map[(pos.0 as i32 + dir.0) as usize][(pos.1 as i32 + dir.1) as usize] {
            Pipe::Pipe(a) => {
                pos.0 = (pos.0 as i32 + dir.0) as usize;
                pos.1 = (pos.1 as i32 + dir.1) as usize;
                if a[0].0 == -dir.0 && a[0].1 == -dir.1 {
                    dir.0 = a[1].0;
                    dir.1 = a[1].1;
                } else if a[1].0 == -dir.0 && a[1].1 == -dir.1 {
                    dir.0 = a[0].0;
                    dir.1 = a[0].1;
                } else {
                    panic!();
                }
            }
            Pipe::Start => break,
            Pipe::None => panic!(),
        }
    }

    println!("{:?}", len/2);
}
