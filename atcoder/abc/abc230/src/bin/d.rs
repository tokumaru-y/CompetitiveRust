#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, D: usize,
        LR: [(usize,usize); N]
    }
    let mut ans = 0;
    let mut broken_right = 0;
    let sort_lr = LR.into_iter().sorted_by_key(|x| x.1);
    for (left,right) in sort_lr.into_iter(){
        if broken_right < left {
            ans += 1;broken_right = right + D - 1;
        }
    }
    println!("{}",ans);
}