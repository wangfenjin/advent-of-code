extern crate regex;

use regex::Regex;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::{thread, time};

fn main() {
    let f = File::open("./src/input.txt").unwrap();
    let mut file = BufReader::new(&f);
    let re = Regex::new(r"^depth: (.*)$").unwrap();
    let re2 = Regex::new(r"^target: (.*),(.*)$").unwrap();
    let mut linenum = 0;
    let mut depth = 0;
    let mut targetx = 0;
    let mut targety = 0;
    for line in file.lines() {
        let l = line.unwrap();
        linenum += 1;
        if linenum == 1 {
            let caps = re.captures(&l).unwrap();
            depth = caps[1].parse::<usize>().unwrap();
        } else {
            let caps = re2.captures(&l).unwrap();
            targetx = caps[1].parse::<usize>().unwrap();
            targety = caps[2].parse::<usize>().unwrap();
        }
    }
    println!("{}, ({},{})", depth, targetx, targety);
    let mut geoindex = Vec::with_capacity(targety + 6);
    let mut levels = Vec::with_capacity(targety + 6);
    for y in 0..targety + 6 {
        geoindex.push(Vec::with_capacity(targetx + 6));
        levels.push(Vec::with_capacity(targetx + 6));
        for x in 0..targetx + 6 {
            geoindex[y].push(0);
            levels[y].push(0);
        }
    }
    for y in 0..targety + 6 {
        for x in 0..targetx + 6 {
            if (x == 0 && y == 0) || (x == targetx && y == targety) {
                geoindex[y][x] = depth % 20183;
            } else if x == 0 {
                geoindex[y][x] = (y * 48271 + depth) % 20183;
            } else if y == 0 {
                geoindex[y][x] = (x * 16807 + depth) % 20183;
            } else {
                geoindex[y][x] = (geoindex[y][x - 1] * geoindex[y - 1][x] + depth) % 20183;
            }
            levels[y][x] = geoindex[y][x] % 3;
        }
    }
    let mut part1 = 0;
    for y in 0..targety + 1 {
        for x in 0..targetx + 1 {
            // if x==0
            // levels[y][x] = (geoindex[y][x]+depth)%20183%3;
            part1 += levels[y][x];

            if x == 0 && y == 0 {
                print!("M");
                continue;
            }
            if x == targetx && y == targety {
                print!("T");
                continue;
            }
            match levels[y][x] {
                0 => print!("."),
                1 => print!("="),
                2 => print!("|"),
                _ => panic!("xxx"),
            }
        }
        println!("");
    }
    println!("part1: {}", part1);

    println!(
        "part2: {}",
        shortest_path(&levels, (0, 0, 1, 0), (targetx as i32, targety as i32))
    );
}

fn shortest_path(
    levels: &Vec<Vec<usize>>,
    source: (i32, i32, i32, i32),
    target: (i32, i32),
) -> i32 {
    let start = (source.0, source.1, source.2);
    let end = (target.0, target.1, 1);
    let one_sec = time::Duration::from_millis(1000);

    let mut state: HashMap<(i32, i32, i32), i32> = HashMap::new();
    state.insert(start, source.3);
    let mut q = VecDeque::new();
    q.push_back(source);
    while !q.is_empty() {
        let (x, y, weapon, minute) = q.pop_front().unwrap();
        let current = (x, y, weapon);

        // println!("current {:?}, state {:?}", current, state.len());
        let best = *state.get(&current).unwrap_or(&0);
        if best > 0 && best < minute {
            continue;
        }
        for k in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let next = (current.0 + k.0, current.1 + k.1);
            if next.0 < 0 || next.1 < 0 || next.0 > target.0 + 5 || next.1 > target.1 + 5 {
                continue;
            }
            if next.0 == 0 && next.1 == 0 {
                continue;
            }
            for (next, v) in get_next(levels, current, next, target) {
                let nminute = minute + v;
                if next == start {
                    continue;
                }
                let best = *state.get(&next).unwrap_or(&0);
                if best == 0 || nminute < best {
                    state.insert(next, nminute);
                    q.push_back((next.0, next.1, next.2, nminute));
                }
            }
        }
        // thread::sleep(one_sec);
    }
    return *state.get(&(target.0, target.1, 1)).unwrap();
}

fn get_next(
    levels: &Vec<Vec<usize>>,
    current: (i32, i32, i32),
    next: (i32, i32),
    target: (i32, i32),
) -> HashMap<(i32, i32, i32), i32> {
    let mut r = HashMap::new();
    let weapon = current.2;
    if next == target {
        if weapon != 1 {
            r.insert((next.0, next.1, 1), 8);
        } else {
            r.insert((next.0, next.1, 1), 1);
        }
        return r;
    }
    match levels[next.1 as usize][next.0 as usize] {
        0 => {
            if weapon == 0 {
                r.insert((next.0, next.1, 1), 8);
                r.insert((next.0, next.1, 2), 8);
            } else {
                r.insert((next.0, next.1, weapon), 1);
            }
        }
        1 => {
            if weapon == 1 {
                r.insert((next.0, next.1, 0), 8);
                r.insert((next.0, next.1, 2), 8);
            } else {
                r.insert((next.0, next.1, weapon), 1);
            }
        }
        2 => {
            if weapon == 2 {
                r.insert((next.0, next.1, 1), 8);
                r.insert((next.0, next.1, 0), 8);
            } else {
                r.insert((next.0, next.1, weapon), 1);
            }
        }
        _ => panic!("xxx"),
    }
    return r;
}
