// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_H&lang=jp
#[allow(unused_imports)]
use std::{
    cmp::Reverse,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
    process::exit,
};

#[warn(dead_code)]
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 10_000_000;
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn make_vec(X: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let len = X.len();

    for bit in (0..(1 << len)) {
        let mut sum_w = 0;
        let mut sum_v = 0;

        for j in 0..len {
            if (bit & (1 << j)) > 0 {
                sum_w += X[j].1;
                sum_v += X[j].0;
            }
        }

        res.push((sum_w, sum_v));
    }

    res.sort_by_key(|&(a, b)| (a, Reverse(b)));

    // ソート済みなresの中で、i<jの時res[i].1>res[j].1である場合jではなくiを選んだほうが最適な価値を得られる。
    let mut max_v = 0;
    for i in 0..(res.len()) {
        max_v = max(max_v, res[i].1);
        res[i].1 = max_v;
    }

    res
}
// x以上の値の中で、最も左側にあるindexを返す。
fn lower_bound<T: std::cmp::PartialOrd>(vec: &Vec<(T, T)>, x: T) -> usize {
    let mut is_ng: isize = -1;
    let mut is_ok: isize = vec.len() as isize;
    while (is_ok - is_ng).abs() > 1 {
        let mid = (is_ok + is_ng) / 2;

        if x <= vec[mid as usize].0 {
            is_ok = mid;
        } else {
            is_ng = mid;
        }
    }

    is_ok as usize
}

// input!マクロ　複数個所には記述しない
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        W: usize,
        VW: [(usize, usize); N]
    }

    let vw1 = make_vec(&VW[..(N / 2)]);
    let vw2 = make_vec(&VW[(N / 2)..]);

    let mut ans = 0;
    for (sw, sv) in vw1.into_iter() {
        if W >= sw {
            let ind = lower_bound(&vw2, W - sw + 1);
            ans = max(ans, sv + vw2[ind - 1].1);
        }
    }

    println!("{}", ans);
}
