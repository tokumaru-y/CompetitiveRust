#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
#[allow(unused_imports)]
use proconio::{input, marker::Chars};
fn main() {
    input!{
        N: usize,
        XL: [[i128; 2]; N],
    }
    let mut right_limit: i128 = -1000_000_001;
    let mut ans = 0;
    let mut arms_range = Vec::new();
    for i in 0..N{
        let left = XL[i][0] - XL[i][1];
        let right = XL[i][0] + XL[i][1];
        arms_range.push((right, left));
    }
    arms_range.sort();
    for arms in arms_range.iter() {
        let left = arms.1;
        let right = arms.0;
        if right_limit > left { continue; }
        ans += 1;
        right_limit = right;
    }
    println!("{}", ans);
}