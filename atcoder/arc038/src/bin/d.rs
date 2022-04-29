#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
#[allow(unused_imports)]
use proconio::{input, marker::Chars};
fn main() {
    input! {
        N: usize,
        WH: [[i128; 2]; N],
    }
    let mut dp = vec![0i128; N+1];
    dp[0] = -200_000;
    let mut hw_list = Vec::new();
    for wh in WH.iter() {
        let w = wh[0];
        let h = wh[1];
        hw_list.push((-h, w));
    }
    hw_list.sort();
    for hw in hw_list.iter() {
        let w = -hw.1;
        let index = dp.binary_search(&w).unwrap_or_else(|x| x);
        dp[index] = w;
    }
    for i in (0..N+1).rev() {
        if dp[i] < 0 {
            println!("{}", i);
            std::process::exit(0);
        }
    }
}