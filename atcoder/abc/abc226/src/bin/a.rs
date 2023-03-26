#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
fn main() {
    input! {
        X: String,
    }
    let mut is_add = false;
    let mut is_next = false;
    for c in X.chars() {
        if is_next {
            if c.to_digit(10).unwrap() >= 5 { is_add = true; }
            break;
        }
        if c == '.' { is_next = true;}
    }
    let mut n = X.parse::<f64>().unwrap() as usize;
    if is_add { n += 1; }
    println!("{}",n);
}