#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Isize1, Usize1},
};
#[allow(unused_imports)]
use std::{
    cmp::Reverse,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    process::exit,
};
use std::{
    fmt::Debug,
    io::{stdout, Write},
    mem::swap,
};
fn main() {
    input! {
        N: usize,
    }
    if N % 2 == 1 {
        println!("");
        return;
    }
    let mut candidate = vec![];
    for b in 0..(1 << N) {
        let mut tmp = vec![];

        for i in 0..N {
            if b & 1 << i > 0 {
                tmp.push(')');
            } else {
                tmp.push('(');
            }
        }
        if is_acceptable(&tmp) {
            candidate.push(tmp)
        }
    }

    candidate.sort();
    for ans in candidate.into_iter() {
        println!("{}", ans.into_iter().collect::<String>());
    }
}

fn is_acceptable(t: &Vec<char>) -> bool {
    let mut left = 0;
    for c in t.iter() {
        if *c == ')' {
            if left == 0 {
                return false;
            }
            left -= 1;
        } else {
            left += 1;
        }
    }

    if left > 0 {
        return false;
    }

    return true;
}
