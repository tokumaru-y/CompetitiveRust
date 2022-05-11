// 参考:https://stackoverflow.com/questions/48575866/how-to-get-the-lower-bound-and-upper-bound-of-an-element-in-a-btreeset
use std::collections::BTreeSet;

fn neighbors(tree: &BTreeSet<isize>, val: isize) -> (Option<&isize>, Option<&isize>) {
    use std::ops::Bound::*;

    let mut before = tree.range((Unbounded, Excluded(val)));
    let mut after = tree.range((Excluded(val), Unbounded));

    (before.next_back(), after.next())
}