#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]

fn dfs(row: usize, L: &Vec<Vec<usize>>, X: usize, ans: &mut usize, value: usize, limit: usize) {
    if row == limit {
        if X == value { *ans += 1; }
        return
    }
    for n in L[row].iter() {
        if X / (*n) < value { continue; }
        dfs(row+1, L, X, ans, value * (*n), limit);
    }
}
fn main() {
    input! {
        N: usize,
        X: usize,
    }
    let mut L = Vec::new();
    for i in 0..N{
        input! {
            l_cnt: usize,
            mut tmp_l: [usize; l_cnt],
        }
        tmp_l.sort();
        L.push(tmp_l);
    }
    let mut ans = 0;
    dfs(0, &L, X, &mut ans, 1, N);
    println!("{}",ans);
}