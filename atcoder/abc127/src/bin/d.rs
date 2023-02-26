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
        M: usize,
        A: [usize; N],
        BC: [(usize, usize); M]
    }
    let mut ans = 0;
    let mut set: BTreeMap<usize, usize> = BTreeMap::new();
    for a in A.into_iter() {
        *set.entry(a).or_default() += 1;
        ans += a;
    }
    let mut sum = ans;

    for (b, c) in BC.into_iter() {
        let mut range = set.range_mut(..(c + 1));
        let mut tmp_cnt = b;
        let mut remove_kev_value = vec![];
        let mut add_key_value = vec![];
        for (k, v) in range.into_iter() {
            if tmp_cnt >= *v {
                sum -= k * *v;
                sum += c * *v;
                tmp_cnt -= *v;
                add_key_value.push((c, *v));
                *v = 0;
            } else {
                *v -= tmp_cnt;
                sum -= k * tmp_cnt;
                sum += c * tmp_cnt;
                add_key_value.push((c, tmp_cnt));

                tmp_cnt = 0;
            }
            remove_kev_value.push((*k, *v));

            if tmp_cnt == 0 {
                break;
            }
        }

        for (k, v) in remove_kev_value.into_iter() {
            if v > 0 {
                let t = set.get_mut(&k).unwrap();
                *t = v;
            } else {
                set.remove(&k);
            }
        }

        for (k, v) in add_key_value.into_iter() {
            *set.entry(k).or_insert(0) += v;
        }
        ans = max(ans, sum);
    }

    println!("{}", ans);
}
