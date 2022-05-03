use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
// #[allow(unused_imports)]
// use proconio::input;
// use proconio::marker::{Usize1, Isize1};
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
struct Edge {
    from: usize,
    to: usize,
    cost: isize,
}
fn main() {
    input!{
        V: usize,
        E: usize,
        r: usize,
        STD: [(usize, usize, isize); E],
    }
    let mut edges = Vec::new();
    for (s,t,d) in STD.iter() {
        let to = *t;let from = *s; let cost = *d;
        edges.push(Edge{ to, from, cost });
    }
    let unreach_value = 1000_000_00;
    let mut dis = vec![unreach_value; V];
    dis[r] = 0;
    for v in 0..V {
        for edge in edges.iter() {
            if dis[edge.from] == unreach_value { continue; }
            if dis[edge.to] > dis[edge.from] + edge.cost {
                dis[edge.to] = dis[edge.from] + edge.cost;
                if v == V -1 { 
                    println!("NEGATIVE CYCLE");
                    std::process::exit(0);
                }
            }
        }
    }
    for d in dis.into_iter() {
        if d == unreach_value {
            println!("INF");
        } else {
            println!("{}", d);
        }
    }
}