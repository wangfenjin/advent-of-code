extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque, BTreeSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::{thread, time};

fn main() {
    assemble();
    return;

    let f = File::open("./src/input.txt").unwrap();
    let mut file = BufReader::new(&f);
    // let mut inputs = vec![];
    let ip = Regex::new(r"^#ip (.*)$").unwrap();

    let re = Regex::new(r"^(.*) (.*) (.*) (.*)$").unwrap();
    // let mut points = vec![];
    let mut linenum = 0;
    let mut maxx = -1000;
    let mut maxy = -1000;
    let mut minx = 1000;
    let mut miny = 1000;

    let one_sec = time::Duration::from_millis(1000);
    let mut ops = HashMap::with_capacity(16);
    ops.insert("addr".to_string(), 0);
    ops.insert("addi".to_string(), 1);
    ops.insert("mulr".to_string(), 2);
    ops.insert("muli".to_string(), 3);
    ops.insert("banr".to_string(), 4);
    ops.insert("bani".to_string(), 5);
    ops.insert("borr".to_string(), 6);
    ops.insert("bori".to_string(), 7);
    ops.insert("setr".to_string(), 8);
    ops.insert("seti".to_string(), 9);
    ops.insert("gtir".to_string(), 10);
    ops.insert("gtri".to_string(), 11);
    ops.insert("gtrr".to_string(), 12);
    ops.insert("eqir".to_string(), 13);
    ops.insert("eqri".to_string(), 14);
    ops.insert("eqrr".to_string(), 15);
    // let mut state = HashMap::new();
    let mut pv = vec![];
    let mut bound = 0;
    for line in file.lines() {
        let l = line.unwrap();
        if linenum == 0 {
            let caps = ip.captures(&l).unwrap();
            bound = caps[1].parse::<usize>().unwrap();
            // println!("{}", next);
            linenum += 1;
            continue;
        }
        let caps = re.captures(&l).unwrap();
        // println!("{:?}", caps);
        let op = caps[1].to_string();
        // println!("{}", op);
        let a = caps[2].parse::<i64>().unwrap();
        let b = caps[3].parse::<i64>().unwrap();
        let c = caps[4].parse::<i64>().unwrap();
        pv.push((op, a, b, c));
        linenum += 1;
    }
    let maxop = pv.len();
        let mut seen = BTreeSet::new();
        let mut prev = 0;
    loop {
        // if least > 1000 {
        //     break;
        // }
        let mut to = [0i64; 6];
        // to[0] = 1;
        let mut from = [0i64; 4];
        // println!("maxop {}", maxop);
        // println!("maxop {:?}", pv);
        // to = [0, 1, 1009, 1008, 1009, 1];
        // let mut jumpd = 0;
        // let mut part2 = false;
        let mut round = 0;
        to[0] = 0;
        while to[bound] >= 0 && (to[bound] as usize) < maxop {
            // if round > 10000 {
            //     break;
            // }
            round += 1;
            if to[bound] == 28 {
                // println!("result{:?}", to);
                let r5 = to[4];
                if seen.is_empty() {
                    println!("part1: {}", r5);
                }
                if !seen.insert(r5) {
                    println!("{}", prev);
                    break;
                }
                prev = r5;
                // break;
            }
            let (op, a, b, c) = pv[to[bound] as usize].clone();
            // println!("{}={:?}-{:?}", round, to[bound], to);
            // if round>1000&& to[2]<=to[3] && to[3] == 10551408 {
            //     to[2] = 10551408+1;
            //     to[bound] = 9;
            //     // round = 0;
            //     if jumpd == 0 {
            //     jumpd = round;
            //     }
            //     continue;
            // }
            // if round>jumpd+1000&& to[4]<=to[3] && to[3] == 10551408 {
            //     to[4] = 10551408+1;
            //     to[bound] = 13;
            //     to[5] = 1;
            //     continue;
            // }
            from[1] = a;
            from[2] = b;
            from[3] = c;
            let op = op.to_string();
            // println!("{}-{}-{:?}", to[bound], op, to);
            match ops.get(&op).unwrap() {
                0 => to[from[3] as usize] = to[from[1] as usize] + to[from[2] as usize],
                1 => {
                    // to[0] = to[0] + to[from[2] as usize];
                    to[from[3] as usize] = to[from[1] as usize] + from[2];
                }
                2 => {
                    // let (tmp, _) = to[from[1] as usize].overflowing_mul(to[from[2] as usize]);
                    let tmp = to[from[1] as usize] * to[from[2] as usize];
                    to[from[3] as usize] = tmp;
                }
                3 => {
                    // let (tmp, _) = to[from[1] as usize].overflowing_mul(from[2]);
                    let tmp = to[from[1] as usize] * from[2];
                    to[from[3] as usize] = tmp;
                }
                4 => to[from[3] as usize] = to[from[1] as usize] & to[from[2] as usize],
                5 => to[from[3] as usize] = to[from[1] as usize] & from[2],
                6 => to[from[3] as usize] = to[from[1] as usize] | to[from[2] as usize],
                7 => to[from[3] as usize] = to[from[1] as usize] | from[2],
                8 => {
                    to[from[3] as usize] = to[from[1] as usize];
                }
                9 => to[from[3] as usize] = from[1],
                10 => to[from[3] as usize] = if from[1] > to[from[2] as usize] { 1 } else { 0 },
                11 => to[from[3] as usize] = if to[from[1] as usize] > from[2] { 1 } else { 0 },
                12 => {
                    to[from[3] as usize] = if to[from[1] as usize] > to[from[2] as usize] {
                        1
                    } else {
                        0
                    }
                }
                13 => {
                    to[from[3] as usize] = if from[1] == to[from[2] as usize] {
                        1
                    } else {
                        0
                    }
                }
                14 => {
                    to[from[3] as usize] = if to[from[1] as usize] == from[2] {
                        1
                    } else {
                        0
                    }
                }
                15 => {
                    to[from[3] as usize] = if to[from[1] as usize] == to[from[2] as usize] {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("xxx"),
            }
            if to[bound] < 0 {
                break;
            }
            // if to[0]+1>=maxop as i32 {
            //     break;
            // }
            to[bound] += 1;
            // next = to[0] as usize;
            // println!("{}-{:?}", next,to);
            // thread::sleep(one_sec);
            // if to[bound] > maxop as i64 {
            //     to[0] = 1;
            //     to[1] = 0;
            //     if part2 {
            //         break;
            //     }
            //     part2 = true;
            //     round = 0;
            // }
            // thread::sleep(one_sec);
        }
        println!("{}-{:?}", round, to);
        break;
    }
}

fn assemble() {
    let mut r0 = 0i64;
    let mut r1 = 0i64;
    let mut r2 = 0i64;
    let mut r3 = 65536i64;
    let mut r4 = 707129i64;
    loop {
        // 6
        r3 = r4 | 65536;
        r4 = 707129;
        loop {
            // 8
            r2 = r3 & 255;
            r4 = (((r4 + r2) & 16777215) * 65899) & 16777215;
            r2 = if 256>r3 { 1 } else { 0 };
            println!("{}-{}-{}", r2, r3, r4);
            if r2 == 1 {
                break;
            }
            // 18
            while r2*256<r3 {
                r2 += 1;
            }
            // r1 =1;
            r2 = 256;
            r3 = r2;
            // println!("{}-{}-{}", r2, r3, r4);
        }
        println!("{}", r4);
        break;
        // 28
        r2 = if r4 == r0 { 1 } else { 0 };
        if r2 == 1 {
            break;
        }
    }
}
