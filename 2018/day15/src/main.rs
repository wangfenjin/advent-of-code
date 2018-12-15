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
    // let mut points = vec![];
    let mut linenum = 0;
    let mut maxx = -1000;
    let mut maxy = -1000;
    let mut minx = 1000;
    let mut miny = 1000;

    let one_sec = time::Duration::from_millis(1000);


    let mut state = HashMap::new();
    let mut person = HashMap::new();
    let mut pv = vec![];
    for line in file.lines() {
        let l = line.unwrap();
        // inputs.push(l.clone());

        let mut iter = l.chars();
        let mut i = 0;
        while let Some(c) = iter.next() {
            let pp = (linenum, i);
            if c == 'G' || c == 'E' {
                person.insert(pp, (c, 200));
                pv.push(pp);
            }
            if c == '#' {
                state.insert(pp, true);
            }
            i+=1;
            // if i==12  {
            //     break;
            // }
        }
        linenum += 1;
    }
    let nperson = person.clone();
    let npv = pv.clone();
    let mut elfv = 2;
    loop {
        person = nperson.clone();
        pv = npv.clone();
        let mut iii = 0;
        let mut win = true;
        elfv += 1;
        loop {
            // if iii > 40 {
            //     break;
            // }
            // for i in 0..33 {
            //     for j in 0..33 {
            //         let p = (i, j);
            //         if state.contains_key(&p) {
            //             print!("#");
            //         } else if let Some(v) = person.get(&p) {
            //             if v.1>0 {
            //             print!("{}", v.0);
            //             } else {
            //                 print!(".");
            //             }
            //         } else {
            //             print!(".");
            //         }
            //         // print!("{}", state[i][j]);
            //     }
            //     println!("");
            // }
            // }
            // println!("{}- {:?}", iii, person);
            // println!("{:?}", pv);
            pv.sort();

            // print!("{}-", iii);
            // for k in &pv {
            //     print!("{:?}-{:?}", k,person.get(k).unwrap());
            // }
            // println!("");
            // thread::sleep(one_sec);

            let mut done = true;
            // let mut tmp = vec![];
            // let mut newperson = HashMap::new();

            let mut allsame = true;
            let mut moved = false;
            for xy in &pv {
                allsame = true;
                moved = false;
                let mut pre = ' ';
                for (_,v) in &person {
                    if v.1>0 {
                        if pre == ' ' {
                            pre = v.0;
                        } else if pre != v.0 {
                            allsame = false;
                            break;
                        }
                    }
                }
                // if allsame {
                //     break;
                // }
                let pi = person.remove(xy).unwrap();
                if pi.1 <= 0 {
                    // person.remove(xy);
                    if pi.0 == 'E' {
                        win = false;
                        break;
                    }
                    continue;
                }
                let att = if pi.0 == 'E' {
                    elfv
                } else {
                    3
                };
                // let mut dis = [[0i32; 8]; 8];
                let mut dis = Vec::new();
                let mut pre = HashMap::new();
                dis.push(xy.to_owned());
                pre.insert(xy.to_owned(), xy.to_owned());

                let mut nv = 0;
                loop {
                    if dis.len()==0 {
                        person.insert(xy.to_owned(), pi.to_owned());
                        break;
                    }
                    let mut newdis = Vec::new();
                    dis.sort();
                    nv += 1;
                    let mut min = -1;
                    let mut minj = xy.to_owned();
                    let mut minm = 0;
                    for k in &dis {
                        for pp in [(k.0-1, k.1), (k.0, k.1-1), (k.0, k.1+1), (k.0+1, k.1)].iter() {
                            if !state.contains_key(pp) && !pre.contains_key(pp) {
                                if let Some(pj) = person.get(&pp) {
                                    if pj.1<=0 {
                                        newdis.push(pp.to_owned());
                                        pre.insert(pp.to_owned(), k.clone());
                                    } else if pj.0 != pi.0 && (min<0 || pj.1<minm) {
                                        min = nv;
                                        minj = pp.to_owned();
                                        minm = pj.1;
                                        // newdis.push(minj);
                                        pre.insert(minj, k.clone());
                                        // break;
                                    }
                                } else {
                                    newdis.push(pp.to_owned());
                                    pre.insert(pp.to_owned(), k.clone());
                                }
                            }
                        }
                        if min > 0 {
                            break;
                        }
                    }
                    dis = newdis;
                    // println!("{}-{:?}", min, dis);
                    if min>0 {
                        // find the target
                        if min == 1 {
                            // shot
                            // println!("shoot {:?}", minj);
                            person.entry(minj).and_modify(|v| *v = (v.0, v.1-att));
                            person.insert(xy.to_owned(), pi.to_owned());
                            // let v = person.remove(&minj).unwrap();
                            // if v.1-3>0 {
                                // newperson.insert(minj.to_owned(), (v.0, v.1-3));
                            // }
                        } else {
                            let mut predd = &minj;
                            let mut pred = &minj;
                            while let Some(pp) = pre.get(pred) {
                                // println!("{:?}-{:?}-{:?}", pred, pp, xy);
                                if pp == xy {
                                    break;
                                } else {
                                    pred = pp;
                                }
                            }
                            // person.remove(xy);
                            // println!("move {:?}", pred);
                            person.insert(pred.to_owned(), pi.to_owned());
                            if (pred.0-minj.0 as i32).abs()+(pred.1-minj.1 as i32).abs() == 1 {
                                // println!("and shot {:?}", minj);

                                person.entry(minj).and_modify(|v| *v = (v.0, v.1-att));
                            }
                        }
                        done = false;
                        moved = true;
                        break;
                    }
                    // dis = newdis;
                }
            }
            if !win {
                break;
            }
            // break;
            if done {
                break;
            }
            pv = person.keys().map(|k| k.to_owned()).collect();
            if !allsame {
                iii += 1;
            }

        }
        if !win {
            continue;
        }
        let mut sum = 0;
        for (_, v) in &person {
            if v.1>0 {
                sum += v.1;
            }
        }
        println!("{}-{}-{}", iii, sum, iii*sum);
        break;
    }
}

struct P {
    C: char,
    S: i32,
    X: i32,
    Y: i32,
}