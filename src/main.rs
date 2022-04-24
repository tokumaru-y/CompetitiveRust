use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut T: usize,
    }
    let MOD = 998244353;
    let mut alpha_tabel = HashMap::new();
    for (i, b) in (b'A'..=b'Z').enumerate() {
        alpha_tabel.insert(char::from(b), i);
    }
    while T > 0 {
        T -= 1;
        input! {
            N: usize,
            S: Chars,
        }
        let mut ans = 0;
        let limit = (N-1)/2;
        for i in 0..=limit {
            ans *= 26;
            ans %= MOD;
            ans += alpha_tabel[&S[i]];
            ans %= MOD;
        }
        let target = S.iter().collect::<String>();
        let mut rev_target = S.to_vec();
        let mut i = 0; let mut j = N-1;
        while i<j {
            rev_target[j] = rev_target[i];
            i+=1;j-=1;
        }
        let rev_target = rev_target.iter().collect::<String>();
        if target >= rev_target {
            ans += 1;
            ans %= MOD;
        }
        println!("{}",ans);
    }
}