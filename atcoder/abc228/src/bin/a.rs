#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
fn main() {
    input! {
        S: usize, T: usize, X: usize,
    }
    let ans = if S > T {
        if S <= X || X < T { "Yes" } else { "No" }
    } else {
        if S <= X && X < T { "Yes" } else { "No" }
    };
    println!("{}",ans);
}