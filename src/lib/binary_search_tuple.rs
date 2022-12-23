// 参考:https://stackoverflow.com/questions/48575866/how-to-get-the-lower-bound-and-upper-bound-of-an-element-in-a-btreeset
use std::collections::BTreeSet;

fn neighbors(tree: &BTreeSet<isize>, val: isize) -> (Option<&isize>, Option<&isize>) {
    use std::ops::Bound::*;

    let mut before = tree.range((Unbounded, Excluded(val)));
    let mut after = tree.range((Excluded(val), Unbounded));

    (before.next_back(), after.next())
}

// x以上の値の中で、最も左側にあるindexを返す。
fn lower_bound<T: std::cmp::PartialOrd>(vec: &Vec<T>, x: T) -> usize {
    let mut is_ng: isize = -1;
    let mut is_ok: isize = vec.len() as isize;
    while (is_ok - is_ng).abs() > 1 {
        let mid = (is_ok + is_ng) / 2;

        if x <= vec[mid as usize] {
            is_ok = mid;
        } else {
            is_ng = mid;
        }
    }

    is_ok as usize
}

// xより大きい要素の一番左のindexを返す。
fn upper_bound<T: std::cmp::PartialOrd>(vec: &Vec<T>, x: T) -> usize {
    let mut is_ng: isize = -1;
    let mut is_ok: isize = vec.len() as isize;
    while (is_ok - is_ng) > 1 {
        let mid = (is_ok + is_ng) / 2;

        if x < vec[mid as usize] {
            is_ok = mid;
        } else {
            is_ng = mid;
        }
    }

    is_ok as usize
}
