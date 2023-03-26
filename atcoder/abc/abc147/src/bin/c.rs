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
    }
    let mut XY = vec![vec![]; N];
    for i in 0..N {
        input! {
            a: usize,
        }
        for _ in 0..a {
            input! {
                x: Usize1,
                y: usize,
            }
            XY[i].push((x, y));
        }
    }

    let mut ans = 0;
    for bit in 0..(1 << N) {
        let mut honest = vec![false; N];

        let mut e_honest = vec![false; N];
        let mut e_unkind = vec![false; N];

        let mut cnt = 0;
        for i in 0..N {
            if bit & (1 << i) > 0 {
                honest[i] = true;
                cnt += 1;
                for (x, y) in XY[i].iter() {
                    if *y == 1 {
                        e_honest[*x] = true;
                    } else {
                        e_unkind[*x] = true;
                    }
                }
            }
        }

        let mut is_ok = true;
        for i in 0..N {
            if e_honest[i] {
                is_ok &= !e_unkind[i] && honest[i];
            }
            if e_unkind[i] {
                is_ok &= !e_honest[i] && !honest[i];
            }
        }
        if is_ok {
            ans = max(ans, cnt);
        }
    }

    println!("{}", ans);
}
