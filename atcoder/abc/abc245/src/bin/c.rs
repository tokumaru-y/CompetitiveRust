use std::{collections::{BinaryHeap, HashMap, VecDeque}, cmp::Reverse};

use proconio::{input, marker::Chars};


fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N],
        B: [usize; N],
    }
    let mut dp = vec![vec![false; 2]; N];
    dp[0][0] = true;
    dp[0][1] = true;
    for i in 0..N-1{
        for j in 0..2 {
            if dp[i][j] {
                let a = if j == 0 { A[i] } else { B[i] };
                for k in 0..2{
                    let b = if k == 0 { A[i+1] } else { B[i+1] };
                    dp[i+1][k] = if (b <= a + K && a <= K + b ) { true } else { dp[i+1][k] }; 
                }
            }
        }
    }
    let ans = if dp[N-1][0] || dp[N-1][1] { "Yes" } else { "No" };
    println!("{}",ans);
}