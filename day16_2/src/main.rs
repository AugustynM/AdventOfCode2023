use std::io::{self, BufRead, BufReader};

enum Tile {
    Empty,
    MirrorPlus,
    MirrorMinus,
    SplitterX,
    SplitterY,
}

fn coords_to_usize(c: (i32, i32)) -> usize {
    if c.0 == 1 && c.1 == 0 {
        return 0;
    }
    if c.0 == -1 && c.1 == 0 {
        return 1;
    }
    if c.0 == 0 && c.1 == 1 {
        return 2;
    }
    if c.0 == 0 && c.1 == -1 {
        return 3;
    }
    panic!();
}

fn go(
    mut c: (usize, usize),
    mut dir: (i32, i32),
    map: &Vec<Vec<Tile>>,
    mut visited: &mut Vec<Vec<Vec<bool>>>,
) {
    loop {
        if visited[c.0][c.1][coords_to_usize(dir)] {
            return;
        }
        visited[c.0][c.1][coords_to_usize(dir)] = true;
        match map[c.0][c.1] {
            Tile::Empty => {
                let x = c.0 as i32 + dir.0;
                let y = c.1 as i32 + dir.1;
                if x < 0 || x >= map.len() as i32 || y < 0 || y >= map[0].len() as i32 {
                    return;
                }
                c = (x as usize, y as usize);
            }
            Tile::MirrorPlus => {
                dir = (dir.1, dir.0);
                let x = c.0 as i32 + dir.0;
                let y = c.1 as i32 + dir.1;
                if x < 0 || x >= map.len() as i32 || y < 0 || y >= map[0].len() as i32 {
                    return;
                }
                c = (x as usize, y as usize);
            }
            Tile::MirrorMinus => {
                dir = (-dir.1, -dir.0);
                let x = c.0 as i32 + dir.0;
                let y = c.1 as i32 + dir.1;
                if x < 0 || x >= map.len() as i32 || y < 0 || y >= map[0].len() as i32 {
                    return;
                }
                c = (x as usize, y as usize);
            }
            Tile::SplitterX => {
                if dir.0 == 0 {
                    let dirc = (dir.1, dir.0);
                    let x = c.0 as i32 + dirc.0;
                    let y = c.1 as i32 + dirc.1;
                    if !(x < 0 || x >= map.len() as i32 || y < 0 || y >= map[0].len() as i32) {
                        let cc = (x as usize, y as usize);
                        go(cc, dirc, &map, &mut visited);
                    }

                    dir = (-dir.1, -dir.0);
                    let x = c.0 as i32 + dir.0;
                    let y = c.1 as i32 + dir.1;
                    if x < 0 || x >= map.len() as i32 || y < 0 || y >= map[0].len() as i32 {
                        return;
                    }
                    c = (x as usize, y as usize);
                } else {
                    let x = c.0 as i32 + dir.0;
                    let y = c.1 as i32 + dir.1;
                    if x < 0 || x >= map.len() as i32 || y < 0 || y >= map[0].len() as i32 {
                        return;
                    }
                    c = (x as usize, y as usize);
                }
            }
            Tile::SplitterY => {
                if dir.1 == 0 {
                    let dirc = (dir.1, dir.0);
                    let x = c.0 as i32 + dirc.0;
                    let y = c.1 as i32 + dirc.1;
                    if !(x < 0 || x >= map.len() as i32 || y < 0 || y >= map[0].len() as i32) {
                        let cc = (x as usize, y as usize);
                        go(cc, dirc, &map, &mut visited);
                    }

                    dir = (-dir.1, -dir.0);
                    let x = c.0 as i32 + dir.0;
                    let y = c.1 as i32 + dir.1;
                    if x < 0 || x >= map.len() as i32 || y < 0 || y >= map[0].len() as i32 {
                        return;
                    }
                    c = (x as usize, y as usize);
                } else {
                    let x = c.0 as i32 + dir.0;
                    let y = c.1 as i32 + dir.1;
                    if x < 0 || x >= map.len() as i32 || y < 0 || y >= map[0].len() as i32 {
                        return;
                    }
                    c = (x as usize, y as usize);
                }
            }
        }
    }
}

fn main() {
    let mut lock = BufReader::new(io::stdin().lock());
    let mut buf = String::new();

    let mut map = Vec::<Vec<Tile>>::new();
    while let Ok(read) = lock.read_line(&mut buf) {
        if read == 0 {
            break;
        }
        let mut line = Vec::<Tile>::new();
        for c in buf.trim_end().chars() {
            line.push(match c {
                '.' => Tile::Empty,
                '\\' => Tile::MirrorPlus,
                '/' => Tile::MirrorMinus,
                '|' => Tile::SplitterX,
                '-' => Tile::SplitterY,
                _ => panic!(),
            });
        }
        map.push(line);
        buf.clear();
    }

    let mut maxsum = 0u32;

    for i in 0..map.len() {
        let mut visited: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; map[0].len()]; map.len()];
        go((i, 0), (0, 1), &map, &mut visited);
        let sum = visited.iter().fold(0u32, |sum, e| {
            sum + e.iter().fold(0u32, |sum, e| match e.contains(&true) {
                true => sum + 1,
                false => sum,
            })
        });
        if maxsum < sum {
            maxsum = sum;
        }

        let mut visited: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; map[0].len()]; map.len()];
        go((i, map[0].len() - 1), (0, -1), &map, &mut visited);
        let sum = visited.iter().fold(0u32, |sum, e| {
            sum + e.iter().fold(0u32, |sum, e| match e.contains(&true) {
                true => sum + 1,
                false => sum,
            })
        });
        if maxsum < sum {
            maxsum = sum;
        }
    }

    for i in 0..map[0].len() {
        let mut visited: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; map[0].len()]; map.len()];
        go((0, i), (1, 0), &map, &mut visited);
        let sum = visited.iter().fold(0u32, |sum, e| {
            sum + e.iter().fold(0u32, |sum, e| match e.contains(&true) {
                true => sum + 1,
                false => sum,
            })
        });
        if maxsum < sum {
            maxsum = sum;
        }

        let mut visited: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; map[0].len()]; map.len()];
        go((map.len() - 1, i), (0, -1), &map, &mut visited);
        let sum = visited.iter().fold(0u32, |sum, e| {
            sum + e.iter().fold(0u32, |sum, e| match e.contains(&true) {
                true => sum + 1,
                false => sum,
            })
        });
        if maxsum < sum {
            maxsum = sum;
        }
    }

    println!("{maxsum}");
}
