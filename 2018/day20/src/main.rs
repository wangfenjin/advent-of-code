extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::{thread, time};

fn main() {
    let f = File::open("./src/input.txt").unwrap();
    let mut file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        let reg = l.chars().collect();
        let mut state = HashMap::new();
        // println!("{:?}", reg);
        state.insert((0, 0), 0);
        let mut part1 = 0;
        let mut part2 = 0;
        shortest(&reg, 1, reg.len()-1, (0, 0), &state, &mut part1, &mut part2);
        println!("{:?}-{}", part1, part2);
    }
}

fn shortest(
    reg: &Vec<char>,
    start: usize,
    end: usize,
    p: (i32, i32),
    state: &HashMap<(i32, i32), i32>,
    largest: &mut i32,
    part2: &mut i32,
) -> Vec<(HashMap<(i32, i32), i32>, (i32, i32))> {
    if start>=end {
        return vec![];
    }
    let mut start = start;
    let mut end = end;

    // find split
    let mut enter = 0;
    let mut split = start;
    let mut spliters = vec![];
    while split < end {
        if reg[split] == '|' {
            if enter == 0 {
            spliters.push(split);
            }
        } else if reg[split] == '(' {
            enter += 1;
        } else if reg[split] == ')' {
            enter -= 1;
        }
        split += 1;
    }
    if spliters.len()>0 {
        // let mut newstart = start;
        let mut rrr = vec![];
        for split in spliters {
            let mut tmp = shortest(reg, start, split, p, state, largest, part2);
            rrr.append(&mut tmp);
            start = split+1;
        }
        let mut tmp = shortest(reg, start, end, p, state, largest, part2);
        rrr.append(&mut tmp);
        return rrr;
    }

    let mut newp = p.clone();
    let mut newstate = state.clone();
    let mut steps = newstate.get(&p).unwrap().to_owned();
    while start < end && reg[start] != '|' && reg[start] != '(' {
        if reg[start] == 'E' {
            newp.0 += 1;
        } else if reg[start] == 'S' {
            newp.1 -= 1;
        } else if reg[start] == 'W' {
            newp.0 -= 1;
        } else if reg[start] == 'N' {
            newp.1 += 1;
        }
        
        // go on
        newstate.entry(newp).and_modify(|e| {
            if steps+1<*e {
                steps += 1;
                if steps<1000 {
                    *part2 -= 1;
                }
                *e = steps;
            } else {
                steps = *e;
            }
        }).or_insert_with(|| {
            steps += 1;
            if steps>=1000 {
                *part2 += 1;
            }
            steps
        });
        if steps>*largest {
            *largest = steps;
        }
        start += 1;
    }

    if start<end && reg[start] == '|' {
        let mut tmpp = shortest(reg, start+1, end, p, state, largest, part2);
        tmpp.push((newstate, newp));
        return tmpp;
    }

    if start<end && reg[start] == '(' {
        let mut subend = start;
        // start += 1;
        let mut level = 0;
        while subend<end {
            if reg[subend] == '(' {
                level += 1;
            } else if reg[subend] == ')' {
                level -= 1;
                if level == 0 {
                    break;
                }
            }
            subend += 1;
        }
        let ps = shortest(reg, start+1, subend, newp, &newstate, largest, part2);
        let mut rrr = vec![];
        for v in ps {
            // println!("for {:?}", v);
            let mut tmp = shortest(reg, subend+1, end, v.1, &v.0, largest, part2);
            rrr.append(&mut tmp);
        }
        return rrr;
    }
    return vec![(newstate, newp)];
}
