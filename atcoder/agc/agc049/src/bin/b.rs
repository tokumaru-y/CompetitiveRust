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
        T: Chars
    }
    let mut s_v = VecDeque::new();
    let mut t_v = VecDeque::new();

    for i in 0..N {
        if S[i] == '1' {
            s_v.push_back(i);
        }
        if T[i] == '1' {
            t_v.push_back(i);
        }
    }

    let mut ans = 0;
    for t in t_v.iter() {
        if s_v.len() == 0 {
            println!("-1");
            exit(0);
        }
        let s = s_v.pop_front().unwrap();
        if *t == s {
            continue;
        }

        if *t > s {
            if s_v.len() < 2 {
                println!("-1");
                exit(0);
            }
            let mut tmp_s = s_v.pop_front().unwrap();
            ans += tmp_s - s;
            let mut tmp_s = s_v.pop_front().unwrap();
            while *t > tmp_s {
                if s_v.len() < 2 {
                    println!("-1");
                    exit(0);
                }
                let _t = s_v.pop_front().unwrap();
                ans += _t - tmp_s;
                tmp_s = s_v.pop_front().unwrap();
            }
            ans += tmp_s - *t;
        } else if *t < s {
            ans += s - *t;
        }
    }

    if s_v.len() > 0 {
        if s_v.len() % 2 == 1 {
            println!("-1");
            exit(0);
        }
        while s_v.len() > 0 {
            let ind = s_v.pop_back().unwrap();
            let n_ind = s_v.pop_back().unwrap();
            ans += ind - n_ind;
        }
    }
    println!("{}", ans);
}
