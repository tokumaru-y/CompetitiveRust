use std::collections::HashSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = 0;
    for a in 1..=10000 {
        for b in a..=(((N/a) as f64).sqrt() as usize) {
            if N / (a * b) >= b { ans += N / (a * b) - b + 1; };
        }
    }
    println!("{}",ans);
}