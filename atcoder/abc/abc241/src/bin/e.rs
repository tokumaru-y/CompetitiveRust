use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
use std::ops::Bound::Included;
use proconio::{input, marker::Chars};

const Limit: usize = 2*10_001;
fn main() {
    input! {
        N: usize,
        mut K: usize,
        A: [usize; N],
    }
    let mut passed_index = vec![Limit; N];
    let mut pre_index = 0;
    let mut sum_list = vec![0usize; N];
    let mut ans:usize = 0;
    let mut cnt = 0;
    while K > 0 {
        let now_index = ans % N;
        if passed_index[now_index] != Limit {
            let circle_cnt = passed_index[pre_index] - passed_index[now_index] + 1;
            let add = sum_list[pre_index] - sum_list[now_index] + A[now_index];
            let div_cnt = K / circle_cnt;
            ans += div_cnt * add;
            K -= div_cnt * circle_cnt;
            while K > 0 {
                ans += A[ans%N];
                K-=1;
            }
        } else {
            passed_index[now_index] = cnt;
            ans += A[now_index];
            sum_list[now_index] += ans;
            pre_index = now_index;
            K -= 1;
        }
        cnt += 1;
    }
    println!("{}",ans);
}