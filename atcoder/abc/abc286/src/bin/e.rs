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
use std::{
    fmt::Debug,
    io::{stdout, Write},
    mem::swap,
};
fn unwrap_result_type<T: Debug>(x: Result<T, T>) -> T {
    if x.is_ok() {
        x.unwrap()
    } else {
        x.unwrap_err()
    }
}

fn read() -> usize {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<usize>().unwrap()
}
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = std::usize::MAX;
const MOD: usize = 1_000_000_007;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        S: [Chars; N],
        Q: usize,
        UV: [(Usize1, Usize1); Q]
    }
    let mut dist = vec![vec![N; N]; N];
    let mut point = vec![vec![0; N]; N];

    for i in 0..N {
        dist[i][i] = 0;
    }

    for i in 0..N {
        for j in 0..N {
            if S[i][j] == 'Y' {
                dist[i][j] = 1;
                point[i][j] = A[j];
            }
        }
    }

    for j in 0..N {
        for i in 0..N {
            for k in 0..N {
                if dist[i][j] + dist[j][k] < dist[i][k] {
                    dist[i][k] = dist[i][j] + dist[j][k];
                    point[i][k] = point[i][j] + point[j][k];
                } else if dist[i][j] + dist[j][k] == dist[i][k]
                    && point[i][j] + point[j][k] > point[i][k]
                {
                    point[i][k] = point[i][j] + point[j][k];
                }
            }
        }
    }

    for q in 0..Q {
        let (u, v) = UV[q];
        if dist[u][v] == N {
            println!("Impossible");
        } else {
            println!("{} {}", dist[u][v], point[u][v] + A[u]);
        }
    }
}
