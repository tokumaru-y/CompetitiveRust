#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
#[allow(unused_imports)]
use proconio::{input, marker::Chars};
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [[usize; 2]; M],
    }
    let mut ans = 0;
    let mut limit = 0;
    let mut islands = Vec::new();
    for ab in AB.iter() {
        islands.push((ab[1], ab[0]));
    }
    islands.sort();
    for island in islands.iter() {
        let left = island.1;
        let right = island.0;
        if left > limit {
            ans += 1;
            limit = right - 1;
        }
    }
    println!("{}",ans);
}