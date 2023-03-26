use std::{collections::{BinaryHeap, HashMap}, cmp::Reverse};

use proconio::{input, marker::Chars};

fn dfs(mut v: Vec<usize>, n: usize, N:usize) -> Vec<usize>{
    if n > N {
        return v;
    }
    let mut nv = Vec::new();
    let v2 = v.to_vec();
    for i in v.into_iter() {
        nv.push(i);
    }
    nv.push(n);
    for i in v2.into_iter() {
        nv.push(i)
    }
    return dfs(nv, n+1, N)
}
fn main() {
    input!{
        N: usize,
    }
    let ans = dfs(Vec::new(), 1, N);
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}