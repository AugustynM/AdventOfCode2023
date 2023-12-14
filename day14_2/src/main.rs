use std::{
    collections::{BTreeSet, HashMap},
    io::{self, BufRead, BufReader},
};

fn main() {
    let mut lock = BufReader::new(io::stdin().lock());
    let mut buf = String::new();
    let mut sum = 0u32;

    let mut round_rocks = BTreeSet::<(u32, u32)>::new();
    let mut cube_rocks = BTreeSet::<(u32, u32)>::new();

    let mut y = 0u32;
    let mut maxx = 0u32;
    while let Ok(read) = lock.read_line(&mut buf) {
        if read == 0 {
            break;
        }
        let mut x = 0u32;
        for ch in buf.trim_end().chars() {
            match ch {
                '.' => {}
                '#' => {
                    cube_rocks.insert((x, y));
                }
                'O' => {
                    round_rocks.insert((x, y));
                }
                _ => panic!(),
            }
            x += 1;
        }
        if maxx < x {
            maxx = x;
        }
        buf.clear();
        y += 1;
    }
    let maxx = maxx;
    let maxy = y;

    let mut memo = HashMap::<BTreeSet<(u32, u32)>, BTreeSet<(u32, u32)>>::new();

    let cycles = 1000000000u32;
    for i in 0..cycles {
        if let Some(s) = memo.get(&round_rocks) {
            let start = s.clone();
            let mut cur = s.clone();
            let mut cl = 0u32;
            loop {
                cur = memo.get(&cur).unwrap().clone();
                cl += 1;
                if cur == start {
                    break;
                }
            }
            let left = (cycles - i-1) % cl;
            for _ in 0..left {
                cur = memo.get(&cur).unwrap().clone();
            }
            round_rocks = cur;
            break;
        }

        let old = round_rocks.clone();

        //north
        let mut temp = BTreeSet::<(u32, u32)>::new();
        for x in 0..maxx {
            let mut w = 0u32;
            for y in 0..maxy {
                if round_rocks.contains(&(x, y)) {
                    temp.insert((x, w));
                    w += 1;
                }
                if cube_rocks.contains(&(x, y)) {
                    w = y + 1;
                }
            }
        }
        round_rocks = temp;

        //west
        let mut temp = BTreeSet::<(u32, u32)>::new();
        for y in 0..maxy {
            let mut w = 0u32;
            for x in 0..maxx {
                if round_rocks.contains(&(x, y)) {
                    temp.insert((w, y));
                    w += 1;
                }
                if cube_rocks.contains(&(x, y)) {
                    w = x + 1;
                }
            }
        }
        round_rocks = temp;

        //south
        let mut temp = BTreeSet::<(u32, u32)>::new();
        for x in 0..maxx {
            let mut w = maxy - 1;
            for y in (0..maxy).rev() {
                if round_rocks.contains(&(x, y)) {
                    temp.insert((x, w));
                    if w > 0 {
                        w -= 1;
                    }
                }
                if cube_rocks.contains(&(x, y)) {
                    if y > 0 {
                        w = y - 1;
                    }
                }
            }
        }
        round_rocks = temp;

        //east
        let mut temp = BTreeSet::<(u32, u32)>::new();
        for y in 0..maxy {
            let mut w = maxx - 1;
            for x in (0..maxx).rev() {
                if round_rocks.contains(&(x, y)) {
                    temp.insert((w, y));
                    if w > 0 {
                        w -= 1;
                    }
                }
                if cube_rocks.contains(&(x, y)) {
                    if x > 0 {
                        w = x - 1;
                    }
                }
            }
        }
        round_rocks = temp;

        memo.insert(old, round_rocks.clone());
    }

    for (_, y) in round_rocks {
        sum += maxy-y;
    }

    println!("{sum}");
}
