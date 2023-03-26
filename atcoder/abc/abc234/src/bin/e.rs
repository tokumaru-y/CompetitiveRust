use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::{Usize1, Isize1};

fn get_number() ->  BTreeSet<usize> {
    let mut res = BTreeSet::<usize>::new();
    for n in 1..10 {
        for d in -9..9 {
            let mut tmp_s = String::from("");
            let mut dg = n;
            for i in 0..18 {
                tmp_s.push_str(&dg.to_string());
                dg+=d;
                res.insert(tmp_s.parse().unwrap());
                if dg < 0 || dg > 9 { break; }
            }
        }
    }
    res
}

fn main() {
    input! {
        X: usize
    }
    let number_list = get_number();
    println!("{}",number_list.range(X..).next().unwrap());
}