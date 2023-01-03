#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Isize1, Usize1},
};
use std::fmt::Debug;
#[allow(unused_imports)]
use std::{
    cmp::Reverse,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    process::exit,
};
fn unwrap_result_type<T: Debug>(x: Result<T, T>) -> T {
    if x.is_ok() {
        x.unwrap()
    } else {
        x.unwrap_err()
    }
}
// x以上の値の中で、最も左側にあるindexを返す。
// vecに対象が存在しない場合はv.lenを返す
fn lower_bound<T: std::cmp::PartialOrd>(vec: &Vec<T>, x: T) -> usize {
    let mut is_ng: isize = -1;
    let mut is_ok: isize = vec.len() as isize;
    while (is_ok - is_ng).abs() > 1 {
        let mid = (is_ok + is_ng) / 2;

        if x <= vec[mid as usize] {
            is_ok = mid;
        } else {
            is_ng = mid;
        }
    }

    is_ok as usize
}
// xより大きい要素の一番左のindexを返す。
// vecに対象が存在しない場合はv.lenを返す
fn upper_bound<T: std::cmp::PartialOrd>(vec: &Vec<T>, x: T) -> usize {
    let mut is_ng: isize = -1;
    let mut is_ok: isize = vec.len() as isize;
    while (is_ok - is_ng) > 1 {
        let mid = (is_ok + is_ng) / 2;

        if x < vec[mid as usize] {
            is_ok = mid;
        } else {
            is_ng = mid;
        }
    }

    is_ok as usize
}
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        sh: usize,
        sw: usize,
        N: usize,
        HW: [(usize, usize); N],
        Q: usize,
        R: [(char, usize); Q]
    }
    let mut hash_h = HashMap::new();
    let mut hash_w = HashMap::new();
    for (h, w) in HW.iter() {
        hash_h.entry(*h).or_insert(vec![]).push(*w);
        hash_w.entry(*w).or_insert(vec![]).push(*h);
    }
    let mut check_h = HashSet::new();
    let mut check_w = HashSet::new();
    for (h, w) in HW.into_iter() {
        if !check_h.contains(&h) {
            check_h.insert(h);
            let mut hv = hash_h.get_mut(&h).unwrap();
            hv.push(0);
            hv.push(W + 1);
            hv.sort();
        }

        if !check_w.contains(&w) {
            check_w.insert(w);
            let mut wv = hash_w.get_mut(&w).unwrap();
            wv.push(0);
            wv.push(H + 1);
            wv.sort();
        }
    }

    let mut now_h = sh;
    let mut now_w = sw;
    for q in (0..Q) {
        let (char, cnt) = R[q];
        if char == 'L' {
            let wv = hash_h.get(&(now_h as usize));
            let mut limit = 0;
            if wv.is_some() {
                let wv = wv.unwrap();
                let ind = lower_bound(wv, now_w as usize);
                if ind > 0 {
                    limit = wv[(ind - 1) as usize];
                }
            }
            if now_w >= cnt {
                now_w = max(now_w - cnt, (limit + 1));
            } else {
                now_w = limit + 1;
            }
        } else if char == 'R' {
            let wv = hash_h.get(&(now_h as usize));
            let mut limit = W + 1;
            if wv.is_some() {
                let wv = wv.unwrap();
                let ind = upper_bound(wv, now_w as usize);
                if ind < wv.len() {
                    limit = wv[ind];
                }
            }
            now_w = min(now_w + cnt, limit - 1);
        } else if char == 'U' {
            let hv = hash_w.get(&(now_w as usize));
            let mut limit = 0;
            if hv.is_some() {
                let hv = hv.unwrap();
                let ind = lower_bound(hv, now_h as usize);
                if ind > 0 {
                    limit = hv[(ind - 1) as usize];
                }
            }
            if now_h >= cnt {
                now_h = max(now_h - cnt, (limit + 1));
            } else {
                now_h = limit + 1;
            }
        } else {
            let hv = hash_w.get(&(now_w as usize));
            let mut limit = H + 1;
            if hv.is_some() {
                let hv = hv.unwrap();
                let ind = upper_bound(hv, now_h as usize);
                if ind < hv.len() {
                    limit = hv[ind];
                }
            }
            now_h = min(now_h + cnt, (limit - 1));
        }

        println!("{} {}", now_h, now_w);
    }
}
