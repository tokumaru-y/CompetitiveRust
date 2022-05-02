use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        N:usize,
        D:[usize; N]
    }
    let mut btree_set = BTreeSet::new();
    for d in D.iter() { btree_set.insert(*d); }
    println!("{}", btree_set.len());
}