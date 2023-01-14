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
        S: Chars
    }
    let mut set = BTreeSet::new();
    let mut even_cnt = 0;
    let mut even_start_ind_list = vec![];
    let mut cnt_table = vec![0; 2 * 10usize.pow(5) + 10];
    // count odds and evens
    for i in 0..(N - 2) {
        if S[i] == 'A' && S[i + 1] == 'R' && S[i + 2] == 'C' {
            even_cnt += 1;
            even_start_ind_list.push(i);
        }
    }

    for ind in 0..(even_start_ind_list.len()) {
        let abc_start_ind = even_start_ind_list[ind];
        if abc_start_ind == 0 {
            continue;
        }
        let mut left_a_cnt = 0;
        let mut right_c_cnt = 0;
        for k in (0..abc_start_ind).rev() {
            if S[k] != 'A' {
                break;
            }
            left_a_cnt += 1;
        }
        for k in (abc_start_ind + 3)..N {
            if S[k] != 'C' {
                break;
            }
            right_c_cnt += 1;
        }
        if left_a_cnt >= 1 && right_c_cnt >= 1 {
            let v = min(left_a_cnt, right_c_cnt) + 1;
            set.insert(v);
            cnt_table[v] += 1;
            even_cnt -= 1;
        }
    }

    // calc ans
    let mut ans = 0;
    for i in 1..(N + 1) {
        if i % 2 == 1 {
            if set.len() == 0 {
                if even_cnt == 0 {
                    break;
                }
                even_cnt -= 1;
            } else {
                let tmp = set.iter().next_back();
                if let Some(&v) = tmp {
                    set.remove(&v);
                    cnt_table[v] -= 1;
                    if cnt_table[v] > 0 {
                        set.insert(v);
                    }
                    if v > 1 {
                        set.insert(v - 1);
                        cnt_table[v - 1] += 1;
                    }
                }
            }
        } else {
            if even_cnt == 0 {
                if set.len() == 0 {
                    break;
                }
                let tmp = set.iter().next();
                if let Some(&v) = tmp {
                    set.remove(&v);
                    cnt_table[v] -= 1;
                    if cnt_table[v] > 0 {
                        set.insert(v);
                    }
                }
            } else {
                even_cnt -= 1;
            }
        }
        ans += 1;
    }
    // print
    println!("{}", ans);
}
