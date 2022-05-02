use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;

fn main() {
    input!{
        N: usize,
        S: [String; N],
        M: usize,
        T: [String; M],
    }
    let mut btree_map = BTreeMap::new();
    for s in S.into_iter() {
        *btree_map.entry(s).or_insert(0) += 1;
    }
    for t in T.into_iter() {
        *btree_map.entry(t).or_insert(0) -= 1;
    }
    let mut ans = 0;
    for (_,v) in btree_map.iter() {
        ans = max(ans, *v);
    }
    println!("{}", ans);
}