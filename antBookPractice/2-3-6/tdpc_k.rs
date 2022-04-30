#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        XR: [[usize;2]; N],
    }
    let mut dp = vec![100_000_0000; N+2];
    dp[0] = -100_000_0000;
    let mut list = Vec::new();
    for xr in XR.iter() {
        let d = xr[0] as i128;let r = xr[1] as i128;
        list.push((r-d,-(d+r)));
    }
    list.sort();
    for l in 0..list.len() {
        let left = list[l].0;let right = -list[l].1;
        let index = dp.binary_search(&right).unwrap_or_else(|x| x);
        dp[index] = right;
    }
    for i in (0..=N).rev() {
        if dp[i] < 100_000_0000 {
            println!("{}", i);
            std::process::exit(0);
        }
    }
}