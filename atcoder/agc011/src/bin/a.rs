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
        N: usize,
        C: usize,
        K: usize,
        mut T: [usize; N]
    }
    T.sort();
    let mut ans = 0;
    let mut deque = VecDeque::from(T);

    while deque.len() > 0 {
        let mut cnt = C;
        let start = deque.pop_front().unwrap();
        let limit = start + K;
        deque.push_front(start);

        while cnt > 0 && deque.len() > 0 {
            let n = deque.pop_front().unwrap();
            if n > limit {
                deque.push_front(n);
                break;
            }
            cnt -= 1;
        }
        ans += 1;
    }

    println!("{}", ans);
}
