extern crate regex;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;
use std::{thread, time};

fn main() {
    let f = File::open("./src/input.txt").unwrap();
    let mut file = BufReader::new(&f);
    // let mut inputs = vec![];
    let re = Regex::new(r"^position=<(.*)> velocity=<(.*)>$").unwrap();
    let mut points = vec![];
    let mut linenum = 0;
    let mut maxx = -1000;
    let mut maxy = -1000;
    let mut minx = 1000;
    let mut miny = 1000;

    let one_sec = time::Duration::from_millis(1000);


    // let mut state = [[0i32; 400]; 400];
    for line in file.lines() {
        let l = line.unwrap();
        // inputs.push(l.clone());
        // linenum += 1;
        // println!("{}", l);
        let caps = re.captures(&l).unwrap();
        // println!("{:?}", caps);
        let xy: Vec<&str> = caps[1].split(",").collect();
        let x = xy[0].trim().parse::<i32>().unwrap();
        let y = xy[1].trim().parse::<i32>().unwrap();
        if x>maxx {
            maxx = x;
        }
        if x<minx {
            minx = x;
        }
        if y>maxy {
            maxy = y;
        }
        if y<miny {
            miny = y;
        }
        let xy: Vec<&str> = caps[2].split(",").collect();
        let vx = xy[0].trim().parse::<i32>().unwrap();
        let vy = xy[1].trim().parse::<i32>().unwrap();
        // println!("{}-{}-{}-{}", x, y, vx, vy);
        points.push(Point{
            X: x,
            Y: y,
            Vx: vx,
            Vy: vy,
        });
    }
    let width = (maxy-miny) as usize;
    let height = (maxx-minx) as usize;
    println!("{}-{}-{}-{}", minx, miny, maxx, maxy);
    // return;
    // let mut state = [[0i8; 100]; 100];
    for c in 0..10000000 {
        let mut nmaxx = -1000000;
        let mut nmaxy = -1000000;
        let mut nminx = 1000000;
        let mut nminy = 1000000;
        for p in &points {     
            let x = p.X-minx+p.Vx*c;
            let y = p.Y-miny+p.Vy*c;
            if x>nmaxx {
                nmaxx = x;
            }
            if x<nminx {
                nminx = x;
            }
            if y>nmaxy {
                nmaxy = y;
            }
            if y<nminy {
                nminy = y;
            }
        }
        let width = (nmaxy-nminy) as usize;
        let height = (nmaxx-nminx) as usize;
        if width>200||height>200 {
            continue;
        }
        let mut state = [[false; 1000]; 1000];
        for p in &points {
            state[(p.X-minx+p.Vx*c-nminx) as usize][(p.Y-miny+p.Vy*c-nminy) as usize] = true;
        }
        println!("");
        println!("{}", c);
        for i in 0..state.len() {
            if i>width {
                break;
            }
            for j in 0..state[i].len() {
                if j>height{
                    break;
                }
                if state[j][i] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        thread::sleep(one_sec);
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    X: i32,
    Y: i32,
    Vx: i32,
    Vy: i32
}
