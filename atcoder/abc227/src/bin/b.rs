use std::collections::HashSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut ans =0;
    let mut hash = HashSet::new();
    for a in 1..1000 {
        for b in 1..1000 {
            let s = 4 * a * b + 3 * a + 3 * b;
            hash.insert(s);
        }
    }
    for a in A.iter() {
        if !hash.contains(a) { ans += 1; }
    }
    println!("{}",ans);
}