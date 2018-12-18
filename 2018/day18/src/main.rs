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
    // let mut inputs = vec![];
    let one_sec = time::Duration::from_millis(1000);
    let re = Regex::new(r"^x=(.*), y=(.*)\.\.(.*)$").unwrap();
    let mut linenum = 0;
    let mut inputs: Vec<Vec<char>> = Vec::new();
    for line in file.lines() {
        let l = line.unwrap();
        linenum += 1;
        let input = l.chars().collect();
        inputs.push(input);
    }
    let mut newi = inputs.clone();
    let mut mm = 0;
    for minute in 0..20000 {
        let maxi = inputs.len();
        for i in 0..maxi {
            let maxj = inputs[i].len();
            for j in 0..maxj {
                if inputs[i][j] == '.' {
                    let mut trees = 0;
                    if i > 0 && j > 0 && inputs[i - 1][j - 1] == '|' {
                        trees += 1;
                    }
                    if i > 0 && inputs[i - 1][j] == '|' {
                        trees += 1;
                    }
                    if i > 0 && j + 1 < maxj && inputs[i - 1][j + 1] == '|' {
                        trees += 1;
                    }
                    if j > 0 && inputs[i][j - 1] == '|' {
                        trees += 1;
                    }
                    if j + 1 < maxj && inputs[i][j + 1] == '|' {
                        trees += 1;
                    }
                    if i + 1 < maxi && j > 0 && inputs[i + 1][j - 1] == '|' {
                        trees += 1;
                    }
                    if i + 1 < maxi && inputs[i + 1][j] == '|' {
                        trees += 1;
                    }
                    if i + 1 < maxi && j + 1 < maxj && inputs[i + 1][j + 1] == '|' {
                        trees += 1;
                    }
                    if trees > 2 {
                        newi[i][j] = '|';
                    }
                } else if inputs[i][j] == '|' {
                    let mut trees = 0;
                    if i > 0 && j > 0 && inputs[i - 1][j - 1] == '#' {
                        trees += 1;
                    }
                    if i > 0 && inputs[i - 1][j] == '#' {
                        trees += 1;
                    }
                    if i > 0 && j + 1 < maxj && inputs[i - 1][j + 1] == '#' {
                        trees += 1;
                    }
                    if j > 0 && inputs[i][j - 1] == '#' {
                        trees += 1;
                    }
                    if j + 1 < maxj && inputs[i][j + 1] == '#' {
                        trees += 1;
                    }
                    if i + 1 < maxi && j > 0 && inputs[i + 1][j - 1] == '#' {
                        trees += 1;
                    }
                    if i + 1 < maxi && inputs[i + 1][j] == '#' {
                        trees += 1;
                    }
                    if i + 1 < maxi && j + 1 < maxj && inputs[i + 1][j + 1] == '#' {
                        trees += 1;
                    }
                    if trees > 2 {
                        newi[i][j] = '#';
                    }
                } else {
                    // #
                    let mut trees = 0;
                    if i > 0 && j > 0 && inputs[i - 1][j - 1] == '|' {
                        trees += 1;
                    }
                    if i > 0 && inputs[i - 1][j] == '|' {
                        trees += 1;
                    }
                    if i > 0 && j + 1 < maxj && inputs[i - 1][j + 1] == '|' {
                        trees += 1;
                    }
                    if j > 0 && inputs[i][j - 1] == '|' {
                        trees += 1;
                    }
                    if j + 1 < maxj && inputs[i][j + 1] == '|' {
                        trees += 1;
                    }
                    if i + 1 < maxi && j > 0 && inputs[i + 1][j - 1] == '|' {
                        trees += 1;
                    }
                    if i + 1 < maxi && inputs[i + 1][j] == '|' {
                        trees += 1;
                    }
                    if i + 1 < maxi && j + 1 < maxj && inputs[i + 1][j + 1] == '|' {
                        trees += 1;
                    }
                    if trees == 0 {
                        newi[i][j] = '.';
                        continue;
                    }
                    let mut trees = 0;
                    if i > 0 && j > 0 && inputs[i - 1][j - 1] == '#' {
                        trees += 1;
                    }
                    if i > 0 && inputs[i - 1][j] == '#' {
                        trees += 1;
                    }
                    if i > 0 && j + 1 < maxj && inputs[i - 1][j + 1] == '#' {
                        trees += 1;
                    }
                    if j > 0 && inputs[i][j - 1] == '#' {
                        trees += 1;
                    }
                    if j + 1 < maxj && inputs[i][j + 1] == '#' {
                        trees += 1;
                    }
                    if i + 1 < maxi && j > 0 && inputs[i + 1][j - 1] == '#' {
                        trees += 1;
                    }
                    if i + 1 < maxi && inputs[i + 1][j] == '#' {
                        trees += 1;
                    }
                    if i + 1 < maxi && j + 1 < maxj && inputs[i + 1][j + 1] == '#' {
                        trees += 1;
                    }
                    if trees == 0 {
                        newi[i][j] = '.';
                        continue;
                    }
                    newi[i][j] = '#';
                }
                // print!("{}", inputs[i][j]);
            }
            // println!("");
        }
        inputs = newi.clone();
        let mut tree = 0;
        let mut lam = 0;
        let maxi = inputs.len();
        for i in 0..maxi {
            let maxj = inputs[i].len();
            for j in 0..maxj {
                if inputs[i][j] == '|' {
                    tree += 1;
                } else if inputs[i][j] == '#' {
                    lam += 1;
                }
            }
        }
        if lam * tree == 192766 {
            mm = minute + 1;
            break;
        }
        // println!("{}={}={}={}", minute,tree, lam, lam * tree);
    }

    println!("{}", (1000000000 - mm) % 28);
    // 192766
    // 190365
    // 184886
    // 179850
    // 181306
    // 177210
    // 175357
    // 168168
    // 170863
    // 166464
    // 170352
    // 169024
    // 173910
    // 172200
    // 178640
    // 179968
    // 185952
    // 187844
    // 194205
    // 194271
    // 198489
    // 196098
    // 199374
    // 196992
    // 199064
    // 194110
    // 196378
    // 191475
    let mut tree = 0;
    let mut lam = 0;
    let maxi = inputs.len();
    for i in 0..maxi {
        let maxj = inputs[i].len();
        for j in 0..maxj {
            if inputs[i][j] == '|' {
                tree += 1;
            } else if inputs[i][j] == '#' {
                lam += 1;
            }
        }
    }
    println!("{}={}={}", tree, lam, lam * tree);
}
