use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
use std::ops::Bound::Included;
use proconio::{input, marker::Chars};

fn dfs(X: &Vec<usize>, to: &Vec<Vec<usize>>, list: &mut Vec<Vec<usize>>, v: usize, p: usize) {
    for i in to[v].iter() {
        if *i == p { continue; }
        dfs(X, to, list, *i, v);
        for j in 0..list[*i].len() {
            let num = list[*i][j];
            list[v].push(num); 
        }
    }
    list[v].push(X[v]);
    list[v].sort();
    list[v].reverse();
    let limit = if list[v].len() >= 20 { 20 } else { list[v].len() };
    list[v] = list[v][0..limit].to_vec();
}

fn main() {
    input! {
        N: usize,
        Q: usize,
        X: [usize; N],
        AB: [[usize; 2]; N-1],
        VK: [[usize; 2]; Q],
    }
    let mut to = vec![Vec::new(); N];
    let mut list = vec![Vec::new(); N];
    for ab in AB.iter() {
        let a = ab[0] - 1;let b = ab[1] - 1;
        to[a].push(b);to[b].push(a);
    }
    dfs(&X, &to, &mut list,0, 2*10_0001);
    for vk in VK.iter() {
        let v = vk[0] - 1;let k = vk[1] - 1;
        println!("{}", list[v][k]);
    }
}