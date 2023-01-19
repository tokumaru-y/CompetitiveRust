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

fn read() -> usize {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<usize>().unwrap()
}
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = std::usize::MAX;
const MOD: usize = 1_000_000_007;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: Chars,
    }

    let mut ans = S.clone();
    let mut left = 0;
    let mut right = N;
    let mut hash = HashMap::new();
    for i in b'a'..=b'z' {
        hash.insert(i as char, VecDeque::new());
    }

    for i in 0..N {
        hash.get_mut(&S[i]).unwrap().push_back(i);
    }

    while left < N {
        let mut p = left;
        for c in b'a'..(ans[left] as u8) {
            let mut deque = hash.get_mut(&(c as char)).unwrap();
            let mut is_cap = false;
            while deque.len() > 0 {
                let ind = deque.pop_back().unwrap();
                if ind <= left || ind >= right {
                    continue;
                }
                p = ind;
                is_cap = true;
                break;
            }
            if is_cap {
                break;
            }
        }
        if p == left {
            left += 1;
            continue;
        }

        ans.swap(left, p);
        left += 1;
        right = p;
    }
    println!("{}", ans.into_iter().join(""))
}
