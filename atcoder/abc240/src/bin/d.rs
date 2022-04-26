use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
use std::ops::Bound::Included;
use proconio::{input, marker::Chars};
fn main() {
    input!{
        N: usize,
        A: [usize; N],
    }
    let mut deque = VecDeque::new();
    let mut ans = 0;
    let mut pre_num = 2*10_0001;
    let mut pre_cnt = 0;
    for i in 0..N{
        ans += 1;
        if pre_num == A[i] {
            pre_cnt += 1;
        } else {
            deque.push_back((pre_num, pre_cnt));
            pre_num = A[i];
            pre_cnt = 1;
        }
        if pre_cnt == pre_num {
            ans -= pre_cnt;
            let next = deque.pop_back().unwrap();
            pre_num = next.0;
            pre_cnt = next.1;
        }
        println!("{}", ans);
    }
}