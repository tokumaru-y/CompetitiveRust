// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_E&lang=jp
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
        S: chars,
        T: chars,
    }
    let s_len = S.len();
    let t_len = T.len();
    let mut dp = vec![vec![FIRST_VALUE; t_len + 1]; s_len + 1];

    dp[0][0] = 0;
    for i in 1..=s_len {
        dp[i][0] = i;
    }
    for i in 1..=t_len {
        dp[0][i] = i;
    }
    for i in 0..s_len {
        for j in 0..t_len {
            dp[i + 1][j + 1] = min(dp[i + 1][j + 1], min(dp[i][j + 1] + 1, dp[i + 1][j] + 1));
            if S[i] == T[j] {
                dp[i + 1][j + 1] = min(dp[i + 1][j + 1], dp[i][j]);
            } else {
                dp[i + 1][j + 1] = min(dp[i + 1][j + 1], dp[i][j] + 1);
            }
        }
    }
    println!("{}", dp[s_len][t_len]);
}
