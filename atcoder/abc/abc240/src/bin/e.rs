use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
use std::ops::Bound::Included;
use proconio::{input, marker::Chars};

fn dfs(V: &Vec<Vec<usize>>, R: &mut Vec<usize>, L: &mut Vec<usize>, n: usize, p: i32, leaf_cnt: &mut usize){
    L[n] = *leaf_cnt;
    for nv in V[n].iter() {
        if p >= 0 && *nv == p as usize { continue; }
        dfs(V, R, L, *nv, n as i32, leaf_cnt);
    }
    if V[n].len() == 1 && p != -1 { *leaf_cnt += 1; }
    R[n] = *leaf_cnt -1;
}

fn main() {
    input! {
        N: usize,
        UV: [[usize; 2]; N-1],
    }
    let mut V = vec![Vec::new(); N];
    for i in 0..N-1 {
        let a = UV[i][0] - 1;
        let b = UV[i][1] - 1;
        V[a].push(b);V[b].push(a);
    }
    let mut R = vec![0usize; N];
    let mut L = vec![0usize; N];
    dfs(&V, &mut R, &mut L, 0, -1, &mut 1);
    for i in 0..N {
        println!("{} {}", L[i], R[i]);
    }
}