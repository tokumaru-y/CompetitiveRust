//https://atcoder.jp/contests/abc061/tasks/abc061_d

// 以下、Wrong Answer 最後まで気づけなかった。
// Bellman-Ford法を使用している。これは「グラフ全体で負の経路が存在するのかどうか」を検出するアルゴリズム
// 今回のように特定の頂点への道筋に負の経路が存在するのかどうかというところの検出には対応できていないので要注意。
// fn main() {
//     input! {
//         N: usize,
//         M: usize,
//         ABC: [(Usize1, Usize1, isize); M]
//     }
//     let mut dist = vec![FIRST_VALUE as isize; N];

//     dist[0] = 0;
//     // 負の閉路検出に注意
//     // このロジックでは1→Nへの道順以外に負の閉路が存在する場合にも検出してしまう。
//     // なので、「Nへの道順の中に負の閉路が存在するかどうか」と判断できる仕組みが必要。
//     for i in (0..N) {
//         for j in (0..M) {
//             let (a, b, c) = ABC[j];

//             let cost = dist[a] - c;
//             if dist[b] > cost {
//                 dist[b] = cost;
//                 if i == N - 1 {
//                     println!("inf");
//                     exit(0);
//                 }
//             }
//         }
//     }

//     println!("{}", dist[N - 1] * -1);
// }

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Isize1, Usize1},
};
#[allow(unused_imports)]
use std::{
    cmp::Reverse,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    process::exit,
};
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000_000_000;
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        ABC: [(Usize1, Usize1, isize); M]
    }
    let mut dist = vec![FIRST_VALUE as isize; N];
    let mut negative = vec![false; N];
    dist[0] = 0;
    for i in (0..N) {
        for j in (0..M) {
            let (a, b, c) = ABC[j];

            if dist[a] == FIRST_VALUE as isize {
                continue;
            }

            let cost = dist[a] - c;
            if dist[b] > cost {
                dist[b] = cost;
            }
        }
    }

    for _ in (0..N) {
        for j in (0..M) {
            let (a, b, c) = ABC[j];

            if dist[a] == FIRST_VALUE as isize {
                continue;
            }

            let cost = dist[a] - c;
            if dist[b] > cost {
                dist[b] = cost;
                negative[b] = true;
            }
        }
    }

    if negative[N - 1] {
        println!("inf");
    } else {
        println!("{}", dist[N - 1] * -1);
    }
}
