#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
#[allow(unused_imports)]
use proconio::{input, marker::Chars};
fn main() {
    input! {
        W: usize,
        N: usize,
        K: usize,
        AB: [[usize; 2]; N],
    }
    let unreach_value = -1;
    let mut dp = vec![vec![unreach_value; K+1]; W+1];
    dp[0][0] = 0;
    for ab in AB.iter() {
        let a = ab[0];let b = ab[1];
        for k in (0..K).rev() {
            for w in (0..=W) {
                if a+w <= W && dp[w][k] != unreach_value {
                    dp[a+w][k+1] = std::cmp::max(dp[a+w][k+1], dp[w][k] + b as isize );
                }
            }
        }
    }
    let mut ans = 0;
    for i in 0..=W{
        for j in 0..=K {
            if dp[i][j] == unreach_value {continue;}
            ans = std::cmp::max(ans, dp[i][j]);
        }
    }
    println!("{}",ans);
}