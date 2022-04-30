#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse,cmp::max};
#[allow(unused_imports)]
use proconio::{input, marker::Chars};
fn main() {
    input! {
        N: usize,
        M: usize,
        ABCW: [[usize; 4]; N],
        XYZ: [[usize; 3]; M],
    }
    let mut dp = vec![vec![vec![0; 102];102];102];
    for abcw in ABCW.iter() {
        let a = abcw[0];let b = abcw[1];let c = abcw[2];let w = abcw[3];
        dp[a][b][c] = std::cmp::max(dp[a][b][c], w);
    }
    for a in 0..=100 {
        for b in 0..=100{
            for c in 0..=100{
                dp[a+1][b+1][c+1] = max(dp[a][b][c], dp[a+1][b+1][c+1]);
                dp[a+1][b][c] = max(dp[a][b][c], dp[a+1][b][c]);
                dp[a+1][b+1][c] = max(dp[a][b][c], dp[a+1][b+1][c]);
                dp[a+1][b][c+1] = max(dp[a][b][c], dp[a+1][b][c+1]);
                dp[a][b+1][c+1] = max(dp[a][b][c], dp[a][b+1][c+1]);
                dp[a][b][c+1] = max(dp[a][b][c], dp[a][b][c+1]);
                dp[a][b+1][c] = max(dp[a][b][c], dp[a][b+1][c]);
            }
        }
    }
    for xyz in XYZ.iter(){
        let x = xyz[0];let y = xyz[1];let z = xyz[2];
        println!("{}", dp[x][y][z]);
    }
}