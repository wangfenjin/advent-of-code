extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::{thread, time};

fn main() {
    let f = File::open("./src/test.txt").unwrap();
    let mut file = BufReader::new(&f);
    // let mut inputs = vec![];
    let re = Regex::new(r"^Before: \[(.*), (.*), (.*), (.*)\]$").unwrap();
    let re2 = Regex::new(r"^After:  \[(.*), (.*), (.*), (.*)\]$").unwrap();
    let re3 = Regex::new(r"^(.*) (.*) (.*) (.*)$").unwrap();
    let mut result = 0;
    let mut y = 0;
    let mut direction = [[' '; 151]; 151];
    let mut linenum = -1;
    let mut from = [0i32; 4];
    let mut to = [0i32; 4];
    let mut ops = vec![];
    let mut op: Vec<HashSet<i32>> = Vec::with_capacity(18);
    for i in 0..16 {
        op.push(HashSet::new());
    }

    let mut finalop = HashMap::new();
    let mut used = HashSet::new();
    for line in file.lines() {
        let l = line.unwrap();
        linenum += 1;

        if linenum > 3171 {
            if linenum == 3172 {
                loop {
                    if finalop.len() == 16 {
                        break;
                    }
                    for i in 0..16 {
                        if op[i].len() > 1 {
                            for v in &used {
                                op[i].remove(v);
                            }
                            // op[i] = op[i].iter().filter(|k| !used.contains(k)).cloned().collect();
                        }
                        if op[i].len() == 1 {
                            let opused = op[i].clone().iter().next().unwrap().to_owned();
                            finalop.insert(i, opused);
                            used.insert(opused);
                        }
                    }
                }
                println!("{:?}", finalop);
                to[0] = 0;
                to[1] = 0;
                to[2] = 0;
                to[3] = 0;
                continue;
            }
            if l.len() == 0 {
                continue;
            }
            let caps = re3.captures(&l).unwrap();
            from[0] = caps[1].parse::<i32>().unwrap();
            from[1] = caps[2].parse::<i32>().unwrap();
            from[2] = caps[3].parse::<i32>().unwrap();
            from[3] = caps[4].parse::<i32>().unwrap();
            let op = from[0] as usize;
            match finalop.get(&op).unwrap() {
                0 => to[from[3] as usize] = to[from[1] as usize] + to[from[2] as usize],
                1 => to[from[3] as usize] = to[from[1] as usize] + from[2],
                2 => to[from[3] as usize] = to[from[1] as usize] * to[from[2] as usize],
                3 => to[from[3] as usize] = to[from[1] as usize] * from[2],
                4 => to[from[3] as usize] = to[from[1] as usize] & to[from[2] as usize],
                5 => to[from[3] as usize] = to[from[1] as usize] & from[2],
                6 => to[from[3] as usize] = to[from[1] as usize] | to[from[2] as usize],
                7 => to[from[3] as usize] = to[from[1] as usize] | from[2],
                8 => to[from[3] as usize] = to[from[1] as usize],
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
                _ => {
                    panic!("xxx")
                }
            }
            println!("{}={:?}", from[0], to);
            continue;
        }
        if linenum % 4 == 0 {
            let caps = re.captures(&l).unwrap();
            from[0] = caps[1].parse::<i32>().unwrap();
            from[1] = caps[2].parse::<i32>().unwrap();
            from[2] = caps[3].parse::<i32>().unwrap();
            from[3] = caps[4].parse::<i32>().unwrap();
        } else if linenum % 4 == 1 {
            let opss = l.split(" ").collect::<Vec<&str>>();
            ops = opss.iter().map(|e| e.parse::<i32>().unwrap()).collect();
        } else if linenum % 4 == 2 {
            let caps = re2.captures(&l).unwrap();
            to[0] = caps[1].parse::<i32>().unwrap();
            to[1] = caps[2].parse::<i32>().unwrap();
            to[2] = caps[3].parse::<i32>().unwrap();
            to[3] = caps[4].parse::<i32>().unwrap();
        } else if linenum % 4 == 3 {
            if op[ops[0] as usize].len() == 1 {
                continue;
            }
            let mut c = 0;
            let mut s = HashSet::new();
            // addr
            if from[ops[1] as usize] + from[ops[2] as usize] == to[ops[3] as usize] {
                c += 1;
                s.insert(0);
            }
            // addi
            if from[ops[1] as usize] + ops[2] == to[ops[3] as usize] {
                c += 1;
                s.insert(1);
            }
            // mulr
            if from[ops[1] as usize] * from[ops[2] as usize] == to[ops[3] as usize] {
                c += 1;
                s.insert(2);
            }
            // muli
            if from[ops[1] as usize] * ops[2] == to[ops[3] as usize] {
                c += 1;
                s.insert(3);
            }
            // banr
            if from[ops[1] as usize] & from[ops[2] as usize] == to[ops[3] as usize] {
                c += 1;
                s.insert(4);
            }
            // bani
            if from[ops[1] as usize] & ops[2] == to[ops[3] as usize] {
                c += 1;
                s.insert(5);
            }
            // borr
            if from[ops[1] as usize] | from[ops[2] as usize] == to[ops[3] as usize] {
                c += 1;
                s.insert(6);
            }
            // bori
            if (from[ops[1] as usize] | ops[2]) == to[ops[3] as usize] {
                c += 1;
                s.insert(7);
            }
            // setr
            if from[ops[1] as usize] == to[ops[3] as usize] {
                c += 1;
                s.insert(8);
            }
            // seti
            if ops[1] == to[ops[3] as usize] {
                c += 1;
                s.insert(9);
            }
            // gtir
            if (ops[1] > from[ops[2] as usize] && to[ops[3] as usize] == 1)
                || (ops[1] <= from[ops[2] as usize] && to[ops[3] as usize] == 0)
            {
                c += 1;
                s.insert(10);
            }
            // gtri
            if (from[ops[1] as usize] > ops[2] && to[ops[3] as usize] == 1)
                || (from[ops[1] as usize] <= ops[2] && to[ops[3] as usize] == 0)
            {
                c += 1;
                s.insert(11);
            }
            // gtrr
            if (from[ops[1] as usize] > from[ops[2] as usize] && to[ops[3] as usize] == 1)
                || (from[ops[1] as usize] <= from[ops[2] as usize] && to[ops[3] as usize] == 0)
            {
                c += 1;
                s.insert(12);
            }
            // eqir
            if (ops[1] == from[ops[2] as usize] && to[ops[3] as usize] == 1)
                || (ops[1] != from[ops[2] as usize] && to[ops[3] as usize] == 0)
            {
                c += 1;
                s.insert(13);
            }
            // eqri
            if (from[ops[1] as usize] == ops[2] && to[ops[3] as usize] == 1)
                || (from[ops[1] as usize] != ops[2] && to[ops[3] as usize] == 0)
            {
                c += 1;
                s.insert(14);
            }
            // eqrr
            if (from[ops[1] as usize] == from[ops[2] as usize] && to[ops[3] as usize] == 1)
                || (from[ops[1] as usize] != from[ops[2] as usize] && to[ops[3] as usize] == 0)
            {
                c += 1;
                s.insert(15);
            }
            if c >= 3 {
                result += 1;
            }
            // let s: HashSet<i32>=s.iter().cloned().collect();
            // if s.len() == 1 {
            //     op[ops[0] as usize] = s;
            //     continue;
            // }
            // println!("{:?}",  op[ops[0] as usize]);
            // println!("{:?}",  s);
            if op[ops[0] as usize].len() == 0 {
                op[ops[0] as usize] = s;
            } else {
                op[ops[0] as usize] = op[ops[0] as usize]
                    .iter()
                    .filter(|k| s.contains(k))
                    .cloned()
                    .collect();
            }
        }
    }
    println!("{:?}", to);
    println!("{}", result);
}
