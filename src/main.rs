use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::{Usize1, Isize1};

fn dfs(used_list: &mut Vec<bool>,A: &Vec<Vec<(usize,usize)>>, n: usize, value: usize, N: &usize) -> usize {
    used_list[n] = true;
    let mut res = 0;
    for (v, index) in A[n].iter() {
        if used_list[*index] { continue; }
        used_list[*index] = true;
        if used_list.iter().filter(|x| **x == true).count() == (*N)*2 {
            used_list[*index] = false;
            return value ^ *v;
        }
        for j in 0..used_list.len() {
            if used_list[j] { continue; }
            res = max(res, dfs(used_list, A, j, value ^ *v, N));
        }
        used_list[*index] = false;
    }
    used_list[n] = false;
    res
}

fn main() {
    input!{
        N: usize,
    }
    let mut A = vec![Vec::new(); 2*N];
    for i in 0..2*N-1 {
        input! {
            a_i: [usize; 2*N-i-1],
        }
        for j in 0..a_i.len() {
            A[i].push((a_i[j], j+i+1));
        }
    }
    let mut ans = 0;
    let mut used_list = vec![false; 2*N];
    ans = dfs(&mut used_list, &A,0, 0,&N);
    println!("{}",ans);
}