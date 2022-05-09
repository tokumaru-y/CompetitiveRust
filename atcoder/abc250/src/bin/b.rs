#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
    }
    let mut origin_row = Vec::new();
    let mut is_white = true;
    let mut tmp_c = Vec::new();
    let mut rev_c = Vec::new();
    for _ in 0..N {
        for i in 0..B {
            if is_white {
                tmp_c.push('.');
                rev_c.push('#');
            } else {
                tmp_c.push('#');
                rev_c.push('.');
            }
        }
        is_white = !is_white;
    }
    let mut is_white = true;
    for _ in 0..N{
        for h in 0..A {
            if is_white {
                origin_row.push(tmp_c.to_vec());
            } else {
                origin_row.push(rev_c.to_vec());
            }
        }
        is_white = !is_white;
    }
    for h in 0..origin_row.len() {
        println!("{}", origin_row[h].iter().join(""));
    }
}