fn dijkstra(
    x: usize,
    graph: &Vec<Vec<(usize, isize, isize)>>,
    cost_graph: &mut Vec<isize>,
    is_yen: bool,
) {
    let mut pq = BinaryHeap::new();
    cost_graph[x] = 0;
    pq.push((0, x));

    while !pq.is_empty() {
        let (_, nx) = pq.pop().unwrap();
        for (y, yen, snuuk) in graph[nx].iter() {
            let c = if is_yen { *yen } else { *snuuk };
            if cost_graph[*y] <= cost_graph[nx] - c {
                continue;
            }
            cost_graph[*y] = cost_graph[nx] - c;
            pq.push((-cost_graph[*y], *y));
        }
    }
}
