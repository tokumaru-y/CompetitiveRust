#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
#[allow(unused_imports)]
use proconio::{input, marker::Chars};
fn main() {
    input! {
        S: Chars,
        T: Chars
    }
    let first_value = vec!['x'; S.len()+1].iter().collect::<String>();
    let mut ans = first_value.clone();
    for i in 0..S.len() {
        if (S[i] == T[0] || S[i] == '?') && (i+T.len()-1 < S.len()) {
            let mut tmp_char = S[..i].to_vec();
            let mut is_ok = true;
            for j in 0..T.len() {
                if S[i+j] == '?' || S[i+j] == T[j] {
                    tmp_char.push(T[j]);
                } else {
                    is_ok = false;
                    break;
                }
            }
            if is_ok {
                for i in i+T.len()..S.len() {
                    tmp_char.push(S[i]);
                }
                let char = tmp_char.iter().map(|x| if *x == '?' { 'a' } else { *x }).collect::<String>();
                ans = std::cmp::min(ans, char);
            }
        }
    }
    if ans == first_value {
        println!("UNRESTORABLE");
    } else {
        println!("{}", ans);
    }
}