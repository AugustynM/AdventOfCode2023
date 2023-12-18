use std::{
    collections::{BTreeSet, BinaryHeap},
    io::{self, BufRead, BufReader},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Line {
    y: i64,
    leftup: bool,
    leftx: i64,
    rightup: bool,
    rightx: i64,
}

fn main() {
    let mut lock = BufReader::new(io::stdin().lock());
    let mut buf = String::new();

    let mut current = (0i64, 0i64);

    let mut input = Vec::<Line>::new();
    let mut prevc = '3';

    let mut sum = 0u64;

    while let Ok(read) = lock.read_line(&mut buf) {
        if read == 0 {
            break;
        }
        let mut parts = buf.split(' ');

        parts.next();
        parts.next();
        let ch = parts
            .next()
            .unwrap()
            .trim()
            .trim_matches('(')
            .trim_matches(')')
            .trim_matches('#');

        let c = ch.chars().nth(5).unwrap();
        let d = i64::from_str_radix(&ch[0..5], 16).unwrap();
        match c {
            '3' => {
                current.1 -= d;
                if let Some(p) = input.last_mut() {
                    if prevc == '2' {
                        p.leftup = true;
                    } else {
                        p.rightup = true;
                    }
                }
                prevc = '3';
            }
            '1' => {
                current.1 += d;
                if let Some(p) = input.last_mut() {
                    if prevc == '2' {
                        p.leftup = false;
                    } else {
                        p.rightup = false;
                    }
                }
                prevc = '1';
            }
            '0' => {
                let old = current.0;
                current.0 += d;
                input.push(Line {
                    y: current.1,
                    leftup: prevc == '1',
                    leftx: old,
                    rightup: false,
                    rightx: current.0,
                });
                prevc = '0';
            }
            '2' => {
                let old = current.0;
                current.0 -= d;
                input.push(Line {
                    y: current.1,
                    leftup: false,
                    leftx: current.0,
                    rightup: prevc == '1',
                    rightx: old,
                });
                prevc = '2';
            }
            _ => panic!(),
        }
        buf.clear();
    }

    let mut pq = BinaryHeap::<Line>::from_iter(input.into_iter());
    let mut cury = pq.peek().unwrap().y;
    let mut ups = BTreeSet::<i64>::new();
    let mut cline = Vec::<(i64, i64)>::new();

    while let Some(l) = pq.pop() {
        if l.y < cury {
            let mut it = ups.iter();
            let mut x = 0i64;
            while let Some(n) = it.next() {
                x -= n;
                x += it.next().unwrap();
                x += 1;
            }
            let y = cury - l.y - 1;
            sum += (x * y) as u64;
            sum += cline.iter().fold(0, |s, a| s + 1 + (a.1 - a.0) as u64);
            cury = l.y;
            cline.clear();
            let mut it = ups.iter();
            while let Some(n) = it.next() {
                cline.push((*n, *it.next().unwrap()));
            }
        }

        if l.leftup {
            ups.insert(l.leftx);
        } else {
            ups.remove(&l.leftx);
        }
        if l.rightup {
            ups.insert(l.rightx);
        } else {
            ups.remove(&l.rightx);
        }

        let mut seg = (l.leftx, l.rightx);
        for a in &cline {
            if (a.0 <= seg.0 && a.1 >= seg.0) || (a.0 <= seg.1 && a.1 >= seg.1) {
                seg.0 = seg.0.min(a.0);
                seg.1 = seg.1.max(a.1);
            }
        }
        cline.retain(|a| a.1 < seg.0 || a.0 > seg.1);
        cline.push(seg);
    }

    sum += cline.iter().fold(0, |s, a| s + 1 + (a.1 - a.0) as u64);

    println!("{sum}");
}
