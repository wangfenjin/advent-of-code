extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::{thread, time};

fn main() {
    let f = File::open("./src/input.txt").unwrap();
    let mut file = BufReader::new(&f);
    // let mut inputs = vec![];
    let re = Regex::new(r"^Step (\w) must be finished before step (\w) can begin.$").unwrap();
    let mut result = 0;
    let mut y = 0;
    let mut direction = [[' '; 151]; 151];
    let mut card = vec![];
    let mut cars = HashMap::new();
    for line in file.lines() {
        let l = line.unwrap();
        let mut iter = l.chars();
        let mut x = 0;
        while let Some(v) = iter.next() {
            direction[y][x] = v;
            if v == '<' || v == '>' || v == '^' || v == 'v' {
                card.push((y,x));
                cars.insert((y,x), (0, v));
                if v == '<' || v == '>' {
                    direction[y][x] = '-';
                } else if v == '^' || v == 'v' {
                    direction[y][x] = '|';
                }
            }
            x += 1;
        }
        y += 1;
    }
    loop {
        card.sort();
        let mut tmp = HashMap::new();
        let mut tmpc = HashMap::new();
        for c in &card {
            if tmpc.contains_key(c) {
                continue;
            }
            let (m, v) = cars.remove(c).unwrap().clone();
            if v == '>' {
                let newl = (c.0, c.1+1);
                if cars.contains_key(&newl) || tmp.contains_key(&newl) || tmpc.contains_key(&newl)  {
                    tmpc.insert(newl, true);
                    continue;
                }
                if direction[c.0][c.1 + 1] == '-' {
                    tmp.insert(
                        newl,
                        (m, '>'),
                    );
                } else if direction[c.0][c.1 + 1] == '\\' {
                    tmp.insert(
                        newl,
                        (m, 'v'),
                    );
                } else if direction[c.0][c.1 + 1] == '/' {
                    tmp.insert(
                        newl,
                        (m, '^'),
                    );
                } else if direction[c.0][c.1 + 1] == '+' {
                    let newv = if m == 0 {
                        '^'
                    } else if m == 1 {
                        '>'
                    } else {
                        'v'
                    };
                    tmp.insert(
                        newl,
                        ((m+1)%3, newv),
                    );
                }
            } else if v == '<' {
                let newl = (c.0, c.1-1);
                if cars.contains_key(&newl) || tmp.contains_key(&newl) || tmpc.contains_key(&newl)  {
                    tmpc.insert(newl.clone(), true);
                    continue;
                }
                if direction[c.0][c.1 - 1] == '-' {
                    tmp.insert(
                        newl,
                        (m, '<'),
                    );
                } else if direction[c.0][c.1 - 1] == '\\' {
                    tmp.insert(
                        newl,
                        (m, '^'),
                    );
                } else if direction[c.0][c.1 - 1] == '/' {
                    tmp.insert(
                        newl,
                        (m, 'v'),
                    );
                } else if direction[c.0][c.1 - 1] == '+' {
                    let newv = if m == 0 {
                        'v'
                    } else if m == 1 {
                        '<'
                    } else {
                        '^'
                    };
                    tmp.insert(
                        newl,
                        ((m+1)%3, newv),
                    );
                }
            } else if v == '^' {
                let newl = (c.0-1, c.1);
                if cars.contains_key(&newl) || tmp.contains_key(&newl) || tmpc.contains_key(&newl) {
                    tmpc.insert(newl.clone(), true);
                    continue;
                }
                if direction[c.0-1][c.1] == '|' {
                    tmp.insert(
                        newl,
                        (m, '^'),
                    );
                } else if direction[c.0-1][c.1] == '\\' {
                    tmp.insert(
                        newl,
                        (m, '<'),
                    );
                } else if direction[c.0-1][c.1] == '/' {
                    tmp.insert(
                        newl,
                        (m, '>'),
                    );
                } else if direction[c.0-1][c.1] == '+' {
                    let newv = if m == 0 {
                        '<'
                    } else if m == 1 {
                        '^'
                    } else {
                        '>'
                    };
                    tmp.insert(
                        newl,
                        ((m+1)%3, newv),
                    );
                }
            } else if v == 'v' {
                let newl = (c.0+1, c.1);
                if cars.contains_key(&newl) || tmp.contains_key(&newl) || tmpc.contains_key(&newl) {
                    tmpc.insert(newl.clone(), true);
                    continue;
                }
                if direction[c.0+1][c.1] == '|' {
                    tmp.insert(
                        newl,
                        (m, 'v'),
                    );
                } else if direction[c.0+1][c.1] == '\\' {
                    tmp.insert(
                        newl,
                        (m, '>'),
                    );
                } else if direction[c.0+1][c.1] == '/' {
                    tmp.insert(
                        newl,
                        (m, '<'),
                    );
                } else if direction[c.0+1][c.1] == '+' {
                    let newv = if m == 0 {
                        '>'
                    } else if m == 1 {
                        'v'
                    } else {
                        '<'
                    };
                    tmp.insert(
                        newl,
                        ((m+1)%3, newv),
                    );
                }
            }
        }

        cars.clear();
        card.clear();
        let mut f = false;
        for (c, v) in &tmp {
            if tmpc.contains_key(&(c.0, c.1)) {
                continue;
            }
            cars.insert(c.clone(), *v);
            card.push(c.clone());
        }
        if cars.len() <= 1 {
            for (c, v) in cars {
                println!("{},{}", c.1, c.0);
            }
            break;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Car {
    // X: usize,
    // Y: usize,
    C: char,
    M: i32,
}
