use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{self, BufRead, BufReader},
};

fn dijkstra(map: &Vec<Vec<u32>>) -> Option<u32> {
    //[x][y][direction][distance straigth]
    let mut distance: Vec<Vec<Vec<Vec<u32>>>> =
        vec![vec![vec![vec![u32::MAX; 4]; 4]; map[0].len()]; map.len()];
    let mut pqueue: BinaryHeap<Reverse<(u32, (usize, usize, usize, usize))>> = BinaryHeap::new();
    pqueue.push(Reverse((0, (0, 0, 1, 0))));
    distance[0][0][0][0] = 0;
    while let Some(Reverse((d, v))) = pqueue.pop() {
        if distance[v.0][v.1][v.2][v.3] < d {
            continue;
        }
        match v.2 {
            0 => {
                if v.0 > 0 && v.3 < 3 {
                    if distance[v.0 - 1][v.1][0][v.3 + 1] > map[v.0 - 1][v.1] + d {
                        distance[v.0 - 1][v.1][0][v.3 + 1] = map[v.0 - 1][v.1] + d;
                        pqueue.push(Reverse((map[v.0 - 1][v.1] + d, (v.0 - 1, v.1, 0, v.3 + 1))));
                    }
                }
                if v.1 > 0 {
                    if distance[v.0][v.1 - 1][3][1] > map[v.0][v.1 - 1] + d {
                        distance[v.0][v.1 - 1][3][1] = map[v.0][v.1 - 1] + d;
                        pqueue.push(Reverse((map[v.0][v.1 - 1] + d, (v.0, v.1 - 1, 3, 1))));
                    }
                }
                if v.1 < map[0].len() - 1 {
                    if distance[v.0][v.1 + 1][1][1] > map[v.0][v.1 + 1] + d {
                        distance[v.0][v.1 + 1][1][1] = map[v.0][v.1 + 1] + d;
                        pqueue.push(Reverse((map[v.0][v.1 + 1] + d, (v.0, v.1 + 1, 1, 1))));
                    }
                }
            }
            1 => {
                if v.0 > 0 {
                    if distance[v.0 - 1][v.1][0][1] > map[v.0 - 1][v.1] + d {
                        distance[v.0 - 1][v.1][0][1] = map[v.0 - 1][v.1] + d;
                        pqueue.push(Reverse((map[v.0 - 1][v.1] + d, (v.0 - 1, v.1, 0, 1))));
                    }
                }
                if v.0 < map.len() - 1 {
                    if distance[v.0 + 1][v.1][2][1] > map[v.0 + 1][v.1] + d {
                        distance[v.0 + 1][v.1][2][1] = map[v.0 + 1][v.1] + d;
                        pqueue.push(Reverse((map[v.0 + 1][v.1] + d, (v.0 + 1, v.1, 2, 1))));
                    }
                }
                if v.1 < map[0].len() - 1 && v.3 < 3 {
                    if distance[v.0][v.1 + 1][1][v.3 + 1] > map[v.0][v.1 + 1] + d {
                        distance[v.0][v.1 + 1][1][v.3 + 1] = map[v.0][v.1 + 1] + d;
                        pqueue.push(Reverse((map[v.0][v.1 + 1] + d, (v.0, v.1 + 1, 1, v.3 + 1))));
                    }
                }
            }
            2 => {
                if v.0 < map.len() - 1 && v.3 < 3 {
                    if distance[v.0 + 1][v.1][2][v.3 + 1] > map[v.0 + 1][v.1] + d {
                        distance[v.0 + 1][v.1][2][v.3 + 1] = map[v.0 + 1][v.1] + d;
                        pqueue.push(Reverse((map[v.0 + 1][v.1] + d, (v.0 + 1, v.1, 2, v.3 + 1))));
                    }
                }
                if v.1 > 0 {
                    if distance[v.0][v.1 - 1][3][1] > map[v.0][v.1 - 1] + d {
                        distance[v.0][v.1 - 1][3][1] = map[v.0][v.1 - 1] + d;
                        pqueue.push(Reverse((map[v.0][v.1 - 1] + d, (v.0, v.1 - 1, 3, 1))));
                    }
                }
                if v.1 < map[0].len() - 1 {
                    if distance[v.0][v.1 + 1][1][1] > map[v.0][v.1 + 1] + d {
                        distance[v.0][v.1 + 1][1][1] = map[v.0][v.1 + 1] + d;
                        pqueue.push(Reverse((map[v.0][v.1 + 1] + d, (v.0, v.1 + 1, 1, 1))));
                    }
                }
            }
            3 => {
                if v.0 > 0 {
                    if distance[v.0 - 1][v.1][0][1] > map[v.0 - 1][v.1] + d {
                        distance[v.0 - 1][v.1][0][1] = map[v.0 - 1][v.1] + d;
                        pqueue.push(Reverse((map[v.0 - 1][v.1] + d, (v.0 - 1, v.1, 0, 1))));
                    }
                }
                if v.0 < map.len() - 1 {
                    if distance[v.0 + 1][v.1][2][1] > map[v.0 + 1][v.1] + d {
                        distance[v.0 + 1][v.1][2][1] = map[v.0 + 1][v.1] + d;
                        pqueue.push(Reverse((map[v.0 + 1][v.1] + d, (v.0 + 1, v.1, 2, 1))));
                    }
                }
                if v.1 > 0 && v.3 < 3 {
                    if distance[v.0][v.1 - 1][3][v.3 + 1] > map[v.0][v.1 - 1] + d {
                        distance[v.0][v.1 - 1][3][v.3 + 1] = map[v.0][v.1 - 1] + d;
                        pqueue.push(Reverse((map[v.0][v.1 - 1] + d, (v.0, v.1 - 1, 3, v.3 + 1))));
                    }
                }
            }
            _ => panic!(),
        }
    }
    let out = distance
        .last()?
        .last()?
        .iter()
        .fold(u32::MAX, |min, a| *a.iter().min().unwrap().min(&min));
    if out < u32::MAX {
        Some(out)
    } else {
        None
    }
}

fn main() {
    let mut lock = BufReader::new(io::stdin().lock());
    let mut buf = String::new();
    let mut map = Vec::<Vec<u32>>::new();

    while let Ok(read) = lock.read_line(&mut buf) {
        if read == 0 {
            break;
        }
        let mut line = Vec::<u32>::new();
        for c in buf.trim_end().chars() {
            line.push(c.to_digit(10).unwrap());
        }
        map.push(line);
        buf.clear();
    }

    if let Some(n) = dijkstra(&map) {
        println!("{n}");
    } else {
        panic!();
    }
}
