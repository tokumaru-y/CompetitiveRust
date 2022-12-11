// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_B&lang=jp
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
        V: usize,
        E: usize,
        R: usize,
        STD: [(usize, usize, isize); E]
    }
    let mut dist = vec![FIRST_VALUE as isize; V];
    let mut graph = vec![vec![]; V];
    dist[R] = 0;

    for (s, t, d) in STD.into_iter() {
        graph[s].push((t, d));
    }

    for i in (0..V) {
        for j in (0..V) {
            for (nv, nc) in graph[j].iter() {
                if dist[j] == FIRST_VALUE as isize {
                    continue;
                }
                let cost = dist[j] + nc;
                if dist[*nv] > cost {
                    dist[*nv] = cost;
                    if i == V - 1 {
                        println!("NEGATIVE CYCLE");
                        exit(0);
                    }
                }
            }
        }
    }
    for v in dist.into_iter() {
        if v == FIRST_VALUE as isize {
            println!("INF");
        } else {
            println!("{}", v);
        }
    }
}
