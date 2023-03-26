use std::collections::HashSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut hash = HashSet::new();
    for _ in 0..N{
        input! {
            L: usize,
            A: [String; L],
        }
        hash.insert(A);
    }
    println!("{}", hash.len());
}