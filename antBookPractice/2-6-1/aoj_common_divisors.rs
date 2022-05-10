#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
// #[allow(unused_imports)]
// use proconio::{input, marker::{Chars,Usize1, Isize1}};
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
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        A: [usize; N],
    }
    let mut gcd_num = 0;
    for i in A.iter() {
        gcd_num = gcd(gcd_num, *i);
    }
    let mut ans = divisor_list(&gcd_num);
    ans.sort();
    for a in ans.into_iter() { println!("{}",a); }
}

fn divisor_list(n: &usize) -> Vec<usize> {
    let mut res = Vec::new();
    for i in 1..=((*n as f64).sqrt() as usize) {
        if n % i == 0 {
            res.push(i);
            if i != n/i {res.push(n/i);}
        }
    }
    res
}

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 { return x; }
    return gcd(y, x%y);
}