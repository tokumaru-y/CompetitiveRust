// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2502
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
// input!マクロ　複数個所には記述しない
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        SLP: [[usize; 3]; N],
        M: usize,
        melodies: [usize; M]
    }
    let mut dp = vec![-1; 400];
    dp[0] = 0;
    for i in 0..=393 {
        if dp[i] == -1 {
            continue;
        }
        for iter in SLP.iter() {
            let s = iter[0];
            let l = iter[1];
            let p = iter[2];
            for k in ((s + i)..=min(l + i, 393)) {
                dp[k] = max(dp[k], dp[i] + p as isize);
            }
        }
    }
    let mut ans = Vec::new();
    for m in melodies.iter() {
        if dp[*m] == -1 {
            println!("-1");
            exit(0);
        }
        ans.push(dp[*m]);
    }
    for a in ans.into_iter() {
        println!("{}", a);
    }
}
