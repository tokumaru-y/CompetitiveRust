#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
const MOD: usize = 998244353;
fn main() {
    input! {
        H: usize,
        W: usize,
        K: usize,
        x1: usize,y1: usize,
        x2: usize,y2: usize,
    }
    let mut dp = vec![vec![0;2];2];
    dp[(x1==x2) as usize][(y1==y2) as usize] = 1;
    for _ in 0..K {
        let mut pre_status = vec![vec![0;2];2];
        std::mem::swap(&mut dp, &mut pre_status);
        for w in 0..2{
            dp[0][w] += pre_status[0][w] * (H-2) % MOD;
            dp[1][w] += pre_status[0][w];
            dp[0][w] += pre_status[1][w] * (H-1) % MOD;
            dp[0][w] %= MOD;dp[1][w] %= MOD;
        }
        for h in 0..2 {
            dp[h][0] += pre_status[h][0] * (W-2) % MOD;
            dp[h][1] += pre_status[h][0];
            dp[h][0] += pre_status[h][1] * (W-1) % MOD;
            dp[h][0] %= MOD;dp[h][1] %= MOD;
        }
    }
    println!("{}", dp[1][1] % MOD);
}