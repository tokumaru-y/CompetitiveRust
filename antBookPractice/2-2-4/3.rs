// https://atcoder.jp/contests/abc005/tasks/abc005_3
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
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
    process::exit,
};
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 10_000_000;
#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
        N: usize,
        A: [usize; N],
        M: usize,
        B: [usize; M]
    }
    let mut deque = VecDeque::new();
    let mut customer = [0; 101];
    for b in B.into_iter() {
        customer[b] += 1;
    }
    let mut takoyaki_ind = 0;
    for time in 1..=100 {
        while takoyaki_ind < N && A[takoyaki_ind] == time {
            deque.push_back(time + T);
            takoyaki_ind += 1;
        }
        if customer[time] > 0 {
            while deque.len() > 0 && customer[time] > 0 {
                let front_value = deque.pop_front().unwrap();
                if front_value >= time {
                    customer[time] -= 1;
                }
            }
            if customer[time] > 0 {
                println!("no");
                exit(0);
            }
        }
    }

    println!("yes");
}
