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
    let re2 = Regex::new(r"^y=(.*), x=(.*)\.\.(.*)$").unwrap();
    let mut linenum = 0;
    let mut ymin = 100000;
    let mut ymax = 0;
    let mut xmax = 0;
    let mut xmin = 100000;
    let mut inputx = Vec::new();
    let mut inputy = Vec::new();
    for line in file.lines() {
        let l = line.unwrap();
        linenum += 1;
        if l.starts_with("x") {
            let caps = re.captures(&l).unwrap();
            let x = caps[1].parse::<i32>().unwrap();
            let y1 = caps[2].parse::<i32>().unwrap();
            let y2 = caps[3].parse::<i32>().unwrap();
            if xmax < x {
                xmax = x;
            }
            if xmin > x {
                xmin = x;
            }
            if ymin > y1 {
                ymin = y1;
            }
            if ymax < y2 {
                ymax = y2;
            }
            inputx.push((x, y1, y2));
        } else if l.starts_with("y") {
            let caps = re2.captures(&l).unwrap();
            let y = caps[1].parse::<i32>().unwrap();
            let x1 = caps[2].parse::<i32>().unwrap();
            let x2 = caps[3].parse::<i32>().unwrap();
            if ymax < y {
                ymax = y;
            }
            if ymin > y {
                ymin = y;
            }
            if xmax < x2 {
                xmax = x2;
            }
            if xmin > x1 {
                xmin = x1;
            }
            inputy.push((y, x1, x2));
        }
    }
    let xmin = xmin as usize - 1;
    let ymin = ymin as usize;
    let ymax = ymax as usize + 1;
    let xmax = xmax as usize + 2;
    let mut state = Vec::with_capacity(ymax);
    for i in 0..ymax {
        state.push(Vec::with_capacity(xmax));
        for j in 0..xmax {
            state[i].push('.');
        }
    }
    for i in inputx {
        let (x, y1, y2) = i;
        for j in y1..y2 + 1 {
            state[j as usize][x as usize] = '#';
        }
    }
    for i in inputy {
        let (y, x1, x2) = i;
        for j in x1..x2 + 1 {
            state[y as usize][j as usize] = '#';
        }
    }
    // for i in ymin..ymax {
    //     for j in xmin..xmax {
    //         print!("{}", state[i][j]);
    //     }
    //     println!("");
    // }
    // let start = (0, 500);
    let mut v = VecDeque::new();
    v.push_back((0, 500));
    while !v.is_empty() {
        let start = v.pop_back().unwrap();
        let mut i = start.0;
        let mut j = start.1;
        // for m in ymin..ymax {
        //     for n in xmin..xmax {
        //         if m==i && j == n {
        //             print!("&");
        //         } else {
        //         print!("{}", state[m][n]);
        //         }
        //     }
        //     println!("");
        // }
        // println!("");
        if i > ymax {
            continue;
        }
        if state[i][j] == '~' {
            continue;
        }
        let mut down = i + 1;
        while down < ymax && state[down][j] == '.' {
            state[down][j] = '|';
            v.push_back((down, j));
            down += 1;
        }
        let i = down;
        if i < ymax && (state[i][j] == '#' || state[i][j] == '~') {
            let mut leftmost = j - 1;
            while leftmost >= xmin
                && (state[i - 1][leftmost] == '.' || state[i - 1][leftmost] == '|')
                && (state[i][leftmost] == '#' || state[i][leftmost] == '~')
            {
                leftmost -= 1;
            }
            let mut rightmost = j + 1;
            while rightmost < xmax
                && (state[i - 1][rightmost] == '.' || state[i - 1][rightmost] == '|')
                && (state[i][rightmost] == '#' || state[i][rightmost] == '~')
            {
                rightmost += 1;
            }
            let s = if leftmost >= xmin
                && state[i - 1][leftmost] == '#'
                && rightmost < xmax
                && state[i - 1][rightmost] == '#'
            {
                '~'
            } else {
                '|'
            };
            state[i - 1][j] = s;
            // left
            let mut left = j - 1;
            while left >= xmin
                && (state[i - 1][left] == '.' || state[i-1][left] == '|')
                // && state[i - 1][left] == '.'
                && (state[i][left] == '#' || state[i][left] == '~')
            {
                if s == '|' && state[i - 1][left] != s {
                    v.push_back((i - 1, left));
                }
                state[i - 1][left] = s;
                left -= 1;
            }
            if left >= xmin && state[i - 1][left] == '.' {
                state[i - 1][left] = '|';
                v.push_back((i - 1, left));
            }
            // right
            let mut right = j + 1;
            while right < xmax
                && (state[i - 1][right] == '.'||state[i - 1][right] == '|')
                // && state[i - 1][right] == '.'
                && (state[i][right] == '#' || state[i][right] == '~')
            {
                if s == '|' && state[i - 1][right] != '|' {
                    v.push_back((i - 1, right));
                }
                state[i - 1][right] = s;
                right += 1;
            }
            if right < xmax && state[i - 1][right] == '.' {
                state[i - 1][right] = '|';
                v.push_back((i - 1, right));
            }
        }
        // let count = 20;
        // let xstart = if j<count || xmin>j-count {
        //     xmin
        // } else {
        //     j-count
        // };
        // let xend = if xmax>j+count {
        //     j+count
        // } else {
        //     xmax 
        // };
        // let ystart = if i<count ||ymin>i-count {
        //     ymin
        // } else {
        //     i-count
        // };
        // let yend = if ymax>i+count {
        //     i+count
        // } else {
        //     ymax 
        // };
        // for i in ystart..yend {
        //     for j in xstart..xend {
        //         print!("{}", state[i][j]);
        //     }
        //     println!("");
        // }
        // println!("");
        // thread::sleep(one_sec);
    }
    let mut result = 0;
    let mut retained = 0;
    for i in ymin..ymax {
        for j in xmin..xmax {
            // print!("{}", state[i][j]);
            if state[i][j] == '~' || state[i][j] == '|' {
                result += 1;
            }
            if state[i][j] == '~' {
                retained += 1;
            }
        }
        // println!("");
    }
    println!("{}-{}", result, retained);
}
