use std::{collections::HashMap, process};
use std::cmp;

use proconio::{input, marker::Chars};
fn main() {
    input! {
        N: usize,
        XY: [[usize; 2]; N],
        S: Chars,
    }
    let mut hash: HashMap<usize, (usize, usize)> = HashMap::new();
    for i in 0..N {
        hash.entry(XY[i][1]).or_insert((1_000_000_001, 0));
        let tuple = hash.get(&XY[i][1]).unwrap();
        if S[i] == 'L' {
            hash.insert(XY[i][1], (tuple.0, cmp::max(tuple.1, XY[i][0])));
        } else {
            hash.insert(XY[i][1], (cmp::min(tuple.0, XY[i][0]), tuple.1));
        }
    }
    for (k, tuple) in hash.iter() {
        if tuple.0 < tuple.1 {
            println!("Yes");
            process::exit(0);
        }
    }
    println!("No");
}