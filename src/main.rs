use std::{collections::{BinaryHeap, HashMap, VecDeque}, cmp::Reverse};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        K: usize,
        XY: [[isize; 2]; N],
    }
    if K == 1 {
        println!("Infinity");
        std::process::exit(0);
    }
    let mut hash = HashMap::<isize, usize>::new();
    let mut tilt = Vec::new();
    for i in 0..N {
        for j in i+1..N{
            let a = &XY[i];
            let b = &XY[j];
            let x = b[0] - a[0];
            let y = b[1] - a[1];
            tilt.push((x,y));
        }
    }
    for i in 0..tilt.len() {
        for j in i+1..tilt.len() {
            let mut cnt = hash.entry(tilt[i].1 * tilt[j].0).or_insert(1);
            *cnt += 1;
        }
    }
    let mut ans = 0;
    println!("{:?}", hash);
    for (k, v) in hash.iter() {
        if *v >= K { ans += 1 ;}
    }
    println!("{}", ans);
}   