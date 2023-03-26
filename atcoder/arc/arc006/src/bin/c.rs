#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
#[allow(unused_imports)]
use proconio::{input, marker::Chars};
fn main() {
    input! {
        N: usize,
        W: [usize; N],
    }
    let mut list = vec![100_001; N];
    for w in W.iter() {
        let index = list.binary_search(w).unwrap_or_else(|x| x);
        list[index] = *w;
    }
    let mut ans = list.iter().filter(|x| **x < 100_001).count();
    println!("{}", ans);
}