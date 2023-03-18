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
fn unwrap_result_type<T: Debug>(x: Result<T, T>) -> T {
    if x.is_ok() {
        x.unwrap()
    } else {
        x.unwrap_err()
    }
}

const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = std::usize::MAX;
const MOD: usize = 1_000_000_007;

#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars
    }
    let mut deque = VecDeque::from(S);
    let mut ans = 0;
    while deque.len() > 1 {
        let f_element = deque.pop_front().unwrap();
        let b_element = deque.pop_back().unwrap();
        if f_element == b_element {
            continue;
        }
        if f_element == 'x' {
            ans += 1;
            deque.push_back(b_element);
        } else if b_element == 'x' {
            ans += 1;
            deque.push_front(f_element);
        } else {
            println!("-1");
            exit(0);
        }
    }

    println!("{}", ans);
}
