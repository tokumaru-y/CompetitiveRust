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
        XY: [(isize, isize); N]
    }
    let mut ans = std::usize::MAX;
    let mut sv = XY.clone();
    sv.sort();

    if N <= 2 {
        println!("1");
        exit(0);
    }

    for i in 0..N {
        for j in (i + 1)..N {
            let mut tmp_cnt = 1;
            let mut a = sv[i];
            let mut b = sv[j];

            let p = b.0 - a.0;
            let q = b.1 - a.1;
            let mut nx = p + b.0;
            let mut ny = q + b.1;

            let mut used = vec![false; N];
            let mut used_cnt = 2;
            used[i] = true;
            used[j] = true;
            while used_cnt < N {
                for k in 0..N {
                    if used[k] {
                        continue;
                    }
                    if sv[k].0 == nx && sv[k].1 == ny {
                        used[k] = true;
                        used_cnt += 1;
                        nx += p;
                        ny += q;
                    }
                }
                if used_cnt < N {
                    used_cnt += 1;
                    tmp_cnt += 1;
                    for k in 0..N {
                        if !used[k] {
                            nx = sv[k].0 + p;
                            ny = sv[k].1 + q;
                            used[k] = true;
                            break;
                        }
                    }
                }
            }

            ans = min(ans, tmp_cnt);
        }
    }

    println!("{}", ans);
}
