use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader},
};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

fn count(
    line: &Vec<Spring>,
    mut line_pos: usize,
    groups: &Vec<usize>,
    mut groups_pos: usize,
    mut checked: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    let mut out = 0u64;
    while line[line_pos] != Spring::Unknown {
        if line[line_pos] == Spring::Damaged {
            if groups_pos == groups.len() {
                return 0;
            }
            for _ in 1..groups[groups_pos] {
                line_pos += 1;
                if line_pos >= line.len() || line[line_pos] == Spring::Operational {
                    return 0;
                }
            }
            groups_pos += 1;
            if line_pos + 1 < line.len() && line[line_pos + 1] == Spring::Damaged {
                return 0;
            }
            line_pos += 1;
            if line_pos == line.len() {
                if groups_pos == groups.len() {
                    return 1;
                } else {
                    return 0;
                }
            }
        }
        line_pos += 1;
        if line_pos == line.len() {
            if groups_pos == groups.len() {
                return 1;
            } else {
                return 0;
            }
        }
    }

    if line_pos + 1 < line.len() {
        if let Some(n) = checked.get(&(line_pos + 1, groups_pos)) {
            out += n;
        } else {
            let a = count(&line, line_pos + 1, &groups, groups_pos, &mut checked);
            checked.insert((line_pos + 1, groups_pos), a);
            out += a;
        }
    }

    if groups_pos < groups.len() {
        for _ in 1..groups[groups_pos] {
            line_pos += 1;
            if line_pos == line.len() || line[line_pos] == Spring::Operational {
                return out;
            }
        }
        groups_pos += 1;
        line_pos += 1;
        if line_pos == line.len() {
            if groups_pos == groups.len() {
                return out + 1;
            } else {
                return out;
            }
        }
        if line[line_pos] == Spring::Damaged {
            return out;
        }
        line_pos += 1;
        if line_pos == line.len() {
            if groups_pos == groups.len() {
                return out + 1;
            } else {
                return out;
            }
        }

        if let Some(n) = checked.get(&(line_pos, groups_pos)) {
            out += n;
        } else {
            let a = count(&line, line_pos, &groups, groups_pos, &mut checked);
            checked.insert((line_pos, groups_pos), a);
            out += a;
        }
    } else {
        while line_pos < line.len() {
            if line[line_pos] == Spring::Damaged {
                return 0;
            }
            line_pos += 1;
        }
        return 1;
    }
    out
}

fn main() {
    let mut lock = BufReader::new(io::stdin().lock());
    let mut buf = String::new();
    let mut sum = 0u64;

    while let Ok(read) = lock.read_line(&mut buf) {
        if read == 0 {
            break;
        }
        let mut line: Vec<Spring> = vec![];
        let mut groups: Vec<usize> = vec![];
        let mut parts = buf.split(' ');

        for c in parts.next().unwrap().chars() {
            match c {
                '.' => line.push(Spring::Operational),
                '#' => line.push(Spring::Damaged),
                '?' => line.push(Spring::Unknown),
                _ => panic!(),
            }
        }

        for a in parts.next().unwrap().split(',') {
            groups.push(a.trim().parse::<usize>().unwrap());
        }

        let linec = line.clone();
        for _ in 0..4 {
            line.push(Spring::Unknown);
            for a in &linec {
                line.push(*a);
            }
        }

        let groupsc = groups.clone();
        for _ in 0..4 {
            for a in &groupsc {
                groups.push(*a);
            }
        }

        let mut checked = HashMap::<(usize, usize), u64>::new();
        sum += count(&line, 0, &groups, 0, &mut checked);

        buf.clear();
    }

    println!("{sum}");
}
