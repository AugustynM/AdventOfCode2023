use std::{
    collections::LinkedList,
    io::{self, BufRead, BufReader},
};

#[derive(Clone, Copy)]
enum Pipe {
    Pipe([(i32, i32); 2]),
    Start,
    None,
}

fn main() {
    let mut lock = BufReader::new(io::stdin().lock());
    let mut buf = String::new();
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
                _ => panic!("{ch}"),
            }
        }
        line.push(Pipe::None);
        map.push(line);

        buf.clear();
    }

    map.push(vec![Pipe::None; map[1].len()]);
    map[0] = vec![Pipe::None; map[1].len()];

    let mut main_loop: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];

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
        main_loop[pos.0][pos.1] = true;
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

    let mut map2: Vec<Vec<bool>> = vec![vec![false; map[0].len() - 1]; map.len() - 1];
    map2[0] = vec![true; map[0].len() - 1];
    map2[map.len() - 2] = vec![true; map[0].len() - 1];
    for x in &mut map2 {
        x[0] = true;
        x[map[0].len() - 2] = true;
    }

    let mut q = LinkedList::<(usize, usize)>::new();
    for a in 0..map2.len() {
        q.push_back((a, 0));
        q.push_back((a, map2[0].len() - 1));
    }
    for a in 1..map2[0].len() - 1 {
        q.push_back((0, a));
        q.push_back((map2.len() - 1, a))
    }

    while let Some(a) = q.pop_front() {
        //up
        if a.0 > 0 {
            let mut addl = true;
            let mut addr = true;
            match map[a.0][a.1] {
                Pipe::Pipe(b) => {
                    if b.contains(&(0, 1)) {
                        addl = false;
                    }
                }
                Pipe::Start => addl = false,
                Pipe::None => (),
            }
            match map[a.0][a.1 + 1] {
                Pipe::Pipe(b) => {
                    if b.contains(&(0, -1)) {
                        addr = false;
                    }
                }
                Pipe::Start => addr = false,
                Pipe::None => (),
            }
            if addl || addr {
                if !map2[a.0 - 1][a.1] {
                    map2[a.0 - 1][a.1] = true;
                    q.push_back((a.0 - 1, a.1));
                }
            }
        }

        //down
        if a.0 < map2.len() - 1 {
            let mut addl = true;
            let mut addr = true;
            match map[a.0 + 1][a.1] {
                Pipe::Pipe(b) => {
                    if b.contains(&(0, 1)) {
                        addl = false;
                    }
                }
                Pipe::Start => addl = false,
                Pipe::None => (),
            }
            match map[a.0 + 1][a.1 + 1] {
                Pipe::Pipe(b) => {
                    if b.contains(&(0, -1)) {
                        addr = false;
                    }
                }
                Pipe::Start => addr = false,
                Pipe::None => (),
            }
            if addl || addr {
                if !map2[a.0 + 1][a.1] {
                    map2[a.0 + 1][a.1] = true;
                    q.push_back((a.0 + 1, a.1));
                }
            }
        }

        //left
        if a.1 > 0 {
            let mut addl = true;
            let mut addr = true;
            match map[a.0][a.1] {
                Pipe::Pipe(b) => {
                    if b.contains(&(1, 0)) {
                        addl = false;
                    }
                }
                Pipe::Start => addl = false,
                Pipe::None => (),
            }
            match map[a.0 + 1][a.1] {
                Pipe::Pipe(b) => {
                    if b.contains(&(-1, 0)) {
                        addr = false;
                    }
                }
                Pipe::Start => addr = false,
                Pipe::None => (),
            }
            if addl || addr {
                if !map2[a.0][a.1 - 1] {
                    map2[a.0][a.1 - 1] = true;
                    q.push_back((a.0, a.1 - 1));
                }
            }
        }

        //right
        if a.1 < map2[0].len() - 1 {
            let mut addl = true;
            let mut addr = true;
            match map[a.0][a.1 + 1] {
                Pipe::Pipe(b) => {
                    if b.contains(&(1, 0)) {
                        addl = false;
                    }
                }
                Pipe::Start => addl = false,
                Pipe::None => (),
            }
            match map[a.0 + 1][a.1 + 1] {
                Pipe::Pipe(b) => {
                    if b.contains(&(-1, 0)) {
                        addr = false;
                    }
                }
                Pipe::Start => addr = false,
                Pipe::None => (),
            }
            if addl || addr {
                if !map2[a.0][a.1 + 1] {
                    map2[a.0][a.1 + 1] = true;
                    q.push_back((a.0, a.1 + 1));
                }
            }
        }
    }
    let mut out = 0u32;

    for x in 1..map.len() - 1 {
        for y in 1..map[0].len() - 1 {
            if !main_loop[x][y] {
                if !(map2[x - 1][y - 1] || map2[x - 1][y] || map2[x][y - 1] || map2[x][y]) {
                    out += 1;
                }
            }
        }
    }

    println!("{out}");
}
