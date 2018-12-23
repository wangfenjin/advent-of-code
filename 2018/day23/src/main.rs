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
    let re = Regex::new(r"^pos=<(.*),(.*),(.*)>, r=(.*)$").unwrap();
    let mut linenum = 0;
    let mut maxr = 0;
    let mut maxp = (0, 0, 0);
    let mut inputs = HashMap::new();
    for line in file.lines() {
        let l = line.unwrap();
        linenum += 1;
        let caps = re.captures(&l).unwrap();
        let x = caps[1].parse::<i64>().unwrap();
        let y = caps[2].parse::<i64>().unwrap();
        let z = caps[3].parse::<i64>().unwrap();
        let r = caps[4].parse::<i64>().unwrap();
        let p = (x, y, z);
        inputs.insert(p.clone(), r);
        if r > maxr {
            maxr = r;
            maxp = p;
        }
    }
    let mut part1 = 0;
    let mut xs = Vec::with_capacity(inputs.len());
    let mut ys = Vec::with_capacity(inputs.len());
    let mut zs = Vec::with_capacity(inputs.len());
    for (k, v) in &inputs {
        if distance(k, &maxp) <= maxr {
            part1 += 1;
        }
        xs.push(k.0);
        ys.push(k.1);
        zs.push(k.2);
    }
    println!("{}", part1);
    let mut xsmax = xs.iter().max().unwrap().clone();
    let mut xsmin = xs.iter().min().unwrap().clone();
    let mut ysmax = ys.iter().max().unwrap().clone();
    let mut ysmin = ys.iter().min().unwrap().clone();
    let mut zsmax = zs.iter().max().unwrap().clone();
    let mut zsmin = zs.iter().min().unwrap().clone();

    let mut start = (0, 0, 0);

    let mut step = 1;
    while step < xsmax - xsmin {
        step *= 2;
    }
    println!(
        "{}-{},{}-{},{}-{},{} ",
        xsmin, xsmax, ysmin, ysmax, zsmin, zsmax, step
    );
    loop {
        let mut part2 = (0, 0, 0);
        let mut part2p = 0;
        let mut part2d = 0;

        let mut x = xsmin;
        while x < xsmax + 1 {
            let mut y = ysmin;
            while y < ysmax + 1 {
                let mut z = zsmin;
                while z < zsmax + 1 {
                    // let p = (maxp.0+x, maxp.1+y, maxp.2+z);
                    let p = (x, y, z);
                    let mut pc = 0;
                    for (k, v) in &inputs {
                        if (distance(&p, k) - *v) / step <= 0 {
                            pc += 1;
                        }
                    }
                    if pc > part2p || (pc == part2p && distance(&p, &start) < part2d) {
                        part2p = pc;
                        part2 = p;
                        part2d = distance(&start, &part2);
                    }
                    z += step;
                }
                y += step;
            }
            x += step;
        }

        if step == 1 {
            println!("result {}-{:?}, {}", part2p, part2, part2d);
            break;
        }
        xsmin = part2.0 - step;
        xsmax = part2.0 + step;
        ysmin = part2.1 - step;
        ysmax = part2.1 + step;
        zsmin = part2.2 - step;
        zsmax = part2.2 + step;
        step /= 2;
    }
}

fn distance(k: &(i64, i64, i64), maxp: &(i64, i64, i64)) -> i64 {
    abs((k.0 - maxp.0)) + abs((k.1 - maxp.1)) + abs((k.2 - maxp.2))
}

fn abs(i: i64) -> i64 {
    i.abs()
}
