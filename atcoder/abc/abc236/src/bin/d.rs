use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::{Usize1, Isize1};


fn dfs(A: &Vec<Vec<usize>>, check_list: &mut Vec<bool>, N: usize, now_cnt: usize, ans: &mut usize){
    let mut min_index = N*10;
    for i in 0..N { 
        if !check_list[i] { min_index = i; break;}
    }
    if min_index == N*10 {
        if *ans < now_cnt { *ans = now_cnt; }
        return ;
    }
    
    check_list[min_index] = true;
    for i in 0..N {
        if !check_list[i] {
            check_list[i] = true;
            dfs(A, check_list, N, A[min_index][i] ^ now_cnt, ans);
            check_list[i] = false;
        }
    }
    check_list[min_index] = false;
}


fn main() {
    input!{
        N: usize,
    }
    let mut A = vec![vec![0; 2*N]; 2*N];
    for i in 0..2*N-1 {
        for j in i+1..2*N {
            input! {
                a: usize,
            }
            A[i][j] = a;
        }
    }
    let mut ans = 0;
    let mut check_list = vec![false; 2*N];
    dfs(&A, &mut check_list, 2*N, 0, &mut ans);
    println!("{}",ans);
}