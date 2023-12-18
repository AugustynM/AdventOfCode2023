use std::{
    collections::{HashSet, VecDeque},
    io::{self, BufRead, BufReader},
};

fn main() {
    let mut lock = BufReader::new(io::stdin().lock());
    let mut buf = String::new();
    let mut points = HashSet::<(i32, i32)>::new();

    let mut current = (0i32, 0i32);
    let mut min = (0i32, 0i32);
    let mut max = (0i32, 0i32);

    while let Ok(read) = lock.read_line(&mut buf) {
        if read == 0 {
            break;
        }
        let mut parts = buf.split(' ');
        let c = parts.next().unwrap().chars().next().unwrap();
        let d = parts.next().unwrap().trim().parse::<u32>().unwrap();
        match c {
            'U' => {
                for _ in 0..d {
                    points.insert(current);
                    current.1 -= 1;
                }
                min.1 = min.1.min(current.1);
            }
            'D' => {
                for _ in 0..d {
                    points.insert(current);
                    current.1 += 1;
                }
                max.1 = max.1.max(current.1);
            }
            'R' => {
                for _ in 0..d {
                    points.insert(current);
                    current.0 += 1;
                }
                max.0 = max.0.max(current.0);
            }
            'L' => {
                for _ in 0..d {
                    points.insert(current);
                    current.0 -= 1;
                }
                min.0 = min.0.min(current.1);
            }
            _ => panic!(),
        }
        buf.clear();
    }

    let mut map: Vec<Vec<u8>> =
        vec![vec![0; (max.1 - min.1 + 3) as usize]; (max.0 - min.0 + 3) as usize];
    for a in points {
        map[(a.0 - min.0 + 1) as usize][(a.1 - min.1 + 1) as usize] = 1;
    }

    let mut q = VecDeque::<(usize, usize)>::new();
    let x = map.len();
    let y = map[0].len();
    for i in 0..x {
        q.push_back((i, 0));
        map[i][0] = 2;
        q.push_back((i, y - 1));
        map[i][y - 1] = 2;
    }
    for i in 0..y {
        q.push_back((0, i));
        map[0][i] = 2;
        q.push_back((x - 1, i));
        map[x - 1][i] = 2;
    }

    while let Some(p) = q.pop_front() {
        if p.0 > 0 && map[p.0 - 1][p.1] == 0 {
            map[p.0 - 1][p.1] = 2;
            q.push_back((p.0 - 1, p.1));
        }
        if p.1 > 0 && map[p.0][p.1 - 1] == 0 {
            map[p.0][p.1 - 1] = 2;
            q.push_back((p.0, p.1 - 1));
        }
        if p.0 < x - 1 && map[p.0 + 1][p.1] == 0 {
            map[p.0 + 1][p.1] = 2;
            q.push_back((p.0 + 1, p.1));
        }
        if p.1 < y - 1 && map[p.0][p.1 + 1] == 0 {
            map[p.0][p.1 + 1] = 2;
            q.push_back((p.0, p.1 + 1));
        }
    }

    let sum = map
        .iter()
        .flatten()
        .fold(0, |s, p| if p < &2 { s +1 } else { s });

    println!("{sum}");
}
