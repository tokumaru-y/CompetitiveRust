#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,mut K: usize,A: usize,
    }
    let mut index = A;
    while K > 0 {
        if index > N { index = 1;}
        K-=1;
        index += 1;
    }
    println!("{}",index-1);
}