use std::{collections::{BinaryHeap, HashMap, VecDeque}, cmp::Reverse};

use proconio::{input, marker::Chars};

fn solve(ans: &Vec<isize>, N: usize, M: usize, A: &Vec<isize>, C: &Vec<isize>) {
    let mut calc = vec![0; N+M+1];
    for a_i in 0..N+1 {
        for b_i in 0..ans.len() {
            calc[a_i+b_i] += A[a_i] * ans[b_i];
        }
    }
    let mut is_ok = true;
    for i in 0..N+M+1{
        if C[i] != calc[i] {is_ok = false;}
    }
    if is_ok {
        println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
        std::process::exit(0);
    }
}

fn dfs(ans: &mut Vec<isize>, N: usize, M: usize, A: &Vec<isize>, C: &Vec<isize>){
    if ans.len() == M+1 {
        solve(ans, N, M, A, C);
        return ;
    }
    for i in -101..101 {
        if ans.len() == M && i == 0 {continue;}
        ans.push(i);
        dfs(ans, N, M, A, C);
        ans.pop();
    }
}

fn main() {
    input!{
        N: usize,
        M: usize,
        A: [isize; N+1],
        mut C: [isize; N+M+1],
    }
    while true {
        let mut ans = Vec::new();
        dfs(&mut ans, N, M, &A, &C);
    }
}