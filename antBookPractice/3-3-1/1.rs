// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A&lang=ja
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
struct SegmentTree {
    node: Vec<usize>,
    init_value: usize,
    leaf_cnt: usize,
}

impl SegmentTree {
    pub fn new(v: Vec<usize>, init_value: usize) -> Self {
        let len = v.len();
        let mut n = 1;
        // 完全木なので、2冪の葉が必要
        while (n < len) {
            n *= 2;
        }
        let mut node = vec![init_value; 4 * n - 1];

        for i in 0..len {
            node[i + n - 1] = v[i];
        }

        for i in (0..=(n - 2)).rev() {
            node[i] = min(node[2 * i + 1], node[2 * i + 2]);
        }

        Self {
            node: node,
            init_value: init_value,
            leaf_cnt: n,
        }
    }

    pub fn update(&mut self, x: usize, val: usize) {
        let mut ind = x + (self.leaf_cnt - 1);
        self.node[ind] = val;

        while (ind > 0) {
            ind = (ind - 1) / 2;
            self.node[ind] = min(self.node[2 * ind + 1], self.node[2 * ind + 2]);
        }
    }

    pub fn query(&self, a: usize, b: usize, ind: usize, l: usize, r: usize) -> usize {
        if (r <= a || b <= l) {
            return self.init_value;
        }

        if (a <= l && r <= b) {
            self.node[ind]
        } else {
            let chil_v1 = self.query(a, b, 2 * ind + 1, l, (l + r) / 2);
            let chil_v2 = self.query(a, b, 2 * ind + 2, (l + r) / 2, r);

            min(chil_v1, chil_v2)
        }
    }
}

// input!マクロ　複数個所には記述しない
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
        cxy: [(usize, usize, usize); Q]
    }
    let init_value = 2usize.pow(31) - 1;
    let v = vec![init_value; N + 1];
    let mut seg = SegmentTree::new(v, init_value);

    for (q, x, y) in cxy.into_iter() {
        if q == 0 {
            seg.update(x, y);
        } else {
            let n = seg.query(x, y + 1, 0, 0, seg.leaf_cnt);
            println!("{}", n);
        }
    }
}
