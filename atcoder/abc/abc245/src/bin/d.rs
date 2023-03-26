use std::{collections::{BinaryHeap, HashMap, VecDeque}, cmp::Reverse};

use proconio::{input, marker::Chars};


fn main() {
    input!{
        N: usize,
        M: usize,
        A: [isize; N+1],
        mut C: [isize; N+M+1],
    }
    let mut ans = vec![0; M+1];
    for b_index in (0..M+1).rev() {
        ans[b_index] = C[b_index + N] / A[N];
        for a_index in 0..N+1 {
            C[a_index + b_index] -= ans[b_index] * A[a_index];
        }
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}