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
        H: usize,
        W: usize,
        S: [Chars; H]
    }
    let mut w_cnt = vec![vec![0; W]; H];
    let mut h_cnt = vec![vec![0; W]; H];

    for h in 0..H {
        let mut m_cnt = 0;
        for w in 0..W {
            if S[h][w] == '#' {
                w_cnt[h][w] = 0;
                m_cnt = 0;
            } else {
                m_cnt += 1;
                w_cnt[h][w] = m_cnt;
            }
        }
        m_cnt = 0;
        for w in (0..W).rev() {
            if w_cnt[h][w] > 0 {
                m_cnt = max(m_cnt, w_cnt[h][w]);
                w_cnt[h][w] = m_cnt;
            } else {
                m_cnt = 0;
            }
        }
    }

    for w in 0..W {
        let mut m_cnt = 0;
        for h in 0..H {
            if S[h][w] == '#' {
                h_cnt[h][w] = 0;
                m_cnt = 0;
            } else {
                m_cnt += 1;
                h_cnt[h][w] = m_cnt;
            }
        }
        m_cnt = 0;
        for h in (0..H).rev() {
            if h_cnt[h][w] > 0 {
                m_cnt = max(m_cnt, h_cnt[h][w]);
                h_cnt[h][w] = m_cnt;
            } else {
                m_cnt = 0;
            }
        }
    }

    let mut ans = 0;
    for h in 0..H {
        for w in 0..W {
            ans = max(ans, h_cnt[h][w] + w_cnt[h][w] - 1);
        }
    }

    println!("{}", ans);
}
