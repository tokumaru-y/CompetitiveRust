use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::{Usize1, Isize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        mut ABC: [(Usize1, Usize1, isize); M],
    }
    for abc in ABC.iter_mut() {
        abc.2 = (-1) * abc.2;
    }
    let mut unreach_value = 1000_000_000_0000;
    let mut dis = vec![unreach_value; N];
    dis[0] = 0;
    for v in 0..2*N {
        for abc in ABC.iter() {
            if dis[abc.0] == unreach_value { continue; }
            if dis[abc.1] > dis[abc.0] + abc.2 {
                dis[abc.1] = dis[abc.0] + abc.2;
                if v == 2*N-1 && abc.1 == N-1 {
                    println!("inf");
                    std::process::exit(0);
                }
            }
        }
    }
    println!("{}",-1 * dis[N-1]);
}