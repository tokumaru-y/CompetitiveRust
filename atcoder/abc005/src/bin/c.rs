#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
#[allow(unused_imports)]
use proconio::{input, marker::Chars};
fn main() {
    input! {
        T: usize,
        N: usize,
        A: [usize; N],
        M: usize,
        B: [usize; M],
    }
    let mut deque = VecDeque::new();
    let mut ab_vec = Vec::new();
    for a in A.iter() {
        ab_vec.push((*a, 1))
    }
    for b in B.iter() {
        ab_vec.push((*b, 2));
    }
    ab_vec.sort();
    for ab in ab_vec.into_iter() {
        if ab.1 == 1 {
            deque.push_back((ab.0 + T));
        } else {
            while true {
                let time = deque.pop_front();
                if time.is_none() {
                    println!("no");
                    std::process::exit(0);
                }
                if ab.0 <= time.unwrap() { break; }
            }
        }
    }
    println!("yes");
}