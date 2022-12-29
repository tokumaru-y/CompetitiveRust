// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_5_D&lang=jp
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
struct BIT {
    // 1-indexed
    node: Vec<usize>,
    N: usize,
}

impl BIT {
    pub fn new(size: usize) -> Self {
        Self {
            node: vec![0; size + 1],
            N: size,
        }
    }

    pub fn add(&mut self, ind: usize, val: usize) {
        if ind == 0 {
            self.node[0] = 1;
            return;
        }

        let mut x = ind;
        while x <= self.N {
            self.node[x] += val;
            x += x & x.wrapping_neg();
        }
    }

    pub fn sum(&self, ind: usize) -> usize {
        if ind == 0 {
            return 0;
        }
        let mut res = 0;
        let mut x = ind;

        while (0 < x) {
            res += self.node[x];
            x -= x & x.wrapping_neg();
        }

        res
    }
}

// input!マクロ　複数個所には記述しない
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let mut bit = BIT::new(10usize.pow(9) + 10);
    let mut ans = 0;

    for (i, a) in A.into_iter().enumerate() {
        ans += i - bit.sum(a + 1);
        bit.add(a + 1, 1);
    }

    println!("{}", ans);
}
