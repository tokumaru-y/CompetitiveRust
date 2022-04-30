#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::max, cmp::min};

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
fn main() {
    input!{
        S: chars,
        T: chars,
    }
    let mut dp = vec![vec![100_001; T.len() + 1]; S.len() + 1];
    dp[0][0] = 0;
    for i in 0..=S.len() {
        for j in 0..=T.len() {
            if i == 0 && j == 0 { continue; }
            if i > 0 && j > 0 {
                if S[i-1] == T[j-1] {
                    dp[i][j] = min(dp[i][j], dp[i-1][j-1])
                } else {
                    dp[i][j] = min(dp[i][j], dp[i-1][j-1] + 1);
                }
            }
            if i > 0 { dp[i][j] = min(dp[i][j], dp[i-1][j] + 1); }
            if j > 0 { dp[i][j] = min(dp[i][j], dp[i][j-1] + 1); }
        }
    }
    println!("{}", dp[S.len()][T.len()]);
}