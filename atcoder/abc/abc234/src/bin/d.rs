use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::{Usize1, Isize1};

fn main() {
    input! {
        N: usize,
        K: usize,
        P: [usize; N],
    }
    let mut p_sorted = P.to_vec();
    p_sorted.sort();
    let mut index = N-K;
    let mut num = p_sorted[index];
    let mut ans = Vec::new();
    let mut can_use = vec![true; N+1];
    can_use[num] = false;
    ans.push(num);
    for i in (K..N).rev() {
        if P[i] >= num {
            while !can_use[p_sorted[index]] { index -= 1; }
            can_use[p_sorted[index]] = false;
            num = p_sorted[index];
        }
        can_use[P[i]] = false;
        ans.push(num);
    }
    for i in (0..ans.len()).rev() {
        println!("{}",ans[i]);
    }
}