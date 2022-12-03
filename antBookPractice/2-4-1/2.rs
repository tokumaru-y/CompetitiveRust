// https://atcoder.jp/contests/arc074/tasks/arc074_b
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
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [isize; 3*N]
    }
    let mut S = vec![0; N * 2 + 1];
    let mut T = vec![0; N * 2 + 1];
    let mut sp = BinaryHeap::new();
    let mut tp = BinaryHeap::new();

    for i in 0..2 * N {
        if i < N {
            S[i + 1] = S[i] + A[i];
            sp.push(-A[i]);
        } else {
            let p_num = sp.pop().unwrap();
            if -p_num < A[i] {
                S[i + 1] = S[i] + A[i] + p_num;
                sp.push(-A[i]);
            } else {
                S[i + 1] = S[i];
                sp.push(p_num);
            }
        }
    }

    for i in ((N + 1)..=3 * N).rev() {
        let t_ind = 3 * N - i;
        if i > 2 * N {
            T[t_ind + 1] = T[t_ind] + A[i - 1];
            tp.push(A[i - 1]);
        } else {
            let t_num = tp.pop().unwrap();
            if t_num > A[i - 1] {
                T[t_ind + 1] = T[t_ind] + A[i - 1] - t_num;
                tp.push(A[i - 1]);
            } else {
                T[t_ind + 1] = T[t_ind];
                tp.push(t_num);
            }
        }
    }
    let mut ans = -1000_000_000_000_000_000;
    for i in N..=2 * N {
        ans = max(ans, S[i] - T[3 * N - i])
    }
    println!("{}", ans);
}
