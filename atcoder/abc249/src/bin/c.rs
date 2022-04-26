use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
use std::ops::Bound::Included;
use proconio::{input, marker::Chars};
fn make_table() -> HashMap<char, usize> {
    let mut table = HashMap::new();
    for (i, j) in (b'a'..=b'z').enumerate() {
        table.insert(char::from(j), i);
    }
    table
}
fn main() {
    input!{
        N: usize,
        K: usize,
        S: [Chars; N],
    }
    let mut ans:usize = 0;
    for bit in (0usize..1<<N) {
        let mut table = make_table();
        let mut cnt_table = vec![0; 26];
        for i in 0..N {
            if bit & (1 << i) > 0 {
                for j in b'a'..=b'z' {
                    for k in 0..S[i].len() {
                        if S[i][k] == char::from(j) {
                            cnt_table[table[&char::from(j)]] += 1;
                            break;
                        }
                    }
                }
            }
        }
        let mut tmp_cnt:usize = 0;
        for c in cnt_table {
            if c == K { tmp_cnt += 1;}
        }
        ans = std::cmp::max(ans, tmp_cnt);
    }
    println!("{}", ans);
}