// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_H&lang=ja
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
struct LazyAddSegmentTree {
    N: usize,
    node: Vec<isize>,
    lazy: Vec<isize>,
}
// 区間に対する加算セグ木
// queryは区間の合計値を返す
impl LazyAddSegmentTree {
    pub fn new(v: Vec<usize>) -> Self {
        let len = v.len();
        let mut n = 1;
        // assert!(v.len() > 1, "Not allowed Vector's length 1");
        while n < len {
            n *= 2;
        }

        let mut node = vec![0; 2 * n - 1];
        let mut lazy = vec![0; 2 * n - 1];

        for i in (0..len) {
            node[i + n - 1] = v[i] as isize;
        }
        if n > 1 {
            for i in (0..(n - 2)).rev() {
                node[i] = node[i * 2 + 1] + node[i * 2 + 2];
            }
        }

        Self {
            N: n,
            node: node,
            lazy: lazy,
        }
    }

    // k番目のノードについて遅延評価
    fn eval(&mut self, k: usize, l: usize, r: usize) {
        if self.lazy[k] != 0 {
            // 末端のノード以外の場合
            if (r - l) > 1 {
                self.lazy[2 * k + 1] += self.lazy[k];
                self.lazy[2 * k + 2] += self.lazy[k];
            }
            self.node[k] += self.lazy[k];
            self.lazy[k] = 0;
        }
    }

    pub fn add(&mut self, a: usize, b: usize, v: isize) {
        self._add(a, b, v, 0, 0, self.N)
    }

    // [a,b)に対して、vを加算する。
    // k:=ノード番号 [l,r):=kのノードがカバーしている範囲
    fn _add(&mut self, a: usize, b: usize, v: isize, k: usize, l: usize, r: usize) {
        self.eval(k, l, r);

        if (b <= l || r <= a) {
            return;
        }

        if (a <= l && r <= b) {
            self.lazy[k] += v;
            self.eval(k, l, r);
        } else {
            self._add(a, b, v, 2 * k + 1, l, (l + r) / 2);
            self._add(a, b, v, 2 * k + 2, (l + r) / 2, r);

            self.node[k] = min(self.node[2 * k + 1], self.node[2 * k + 2]);
        }
    }

    pub fn query(&mut self, a: usize, b: usize) -> isize {
        self._query(a, b, 0, 0, self.N)
    }

    fn _query(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> isize {
        if (b <= l || r <= a) {
            return 10isize.pow(10);
        }

        self.eval(k, l, r);
        if (a <= l && r <= b) {
            return self.node[k];
        }

        let sum_left = self._query(a, b, 2 * k + 1, l, (l + r) / 2);
        let sum_right = self._query(a, b, 2 * k + 2, (l + r) / 2, r);

        min(sum_left, sum_right)
    }
}
#[allow(non_snake_case)]
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    let nq: Vec<usize> = buf
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    let N = nq[0];
    let Q = nq[1];

    let mut seg = LazyAddSegmentTree::new(vec![0; N]);

    for _ in (0..Q) {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).ok();
        let qu: Vec<isize> = buf
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();

        if qu[0] == 0 {
            seg.add(qu[1] as usize, qu[2] as usize + 1, qu[3]);
        } else {
            let s = seg.query(qu[1] as usize, qu[2] as usize + 1);
            println!("{}", s);
        }
    }
}
