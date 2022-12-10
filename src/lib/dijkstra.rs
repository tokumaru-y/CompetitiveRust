fn dijkstra(x: usize, graph: &Vec<Vec<(usize, isize)>>, cost_graph: &mut Vec<isize>) {
    let mut pq = BinaryHeap::new();
    cost_graph[x] = 0;
    pq.push((0, x));

    while !pq.is_empty() {
        let (_, nx) = pq.pop().unwrap();
        for (y, cost) in graph[nx].iter() {
            let c = *cost;
            if cost_graph[*y] <= cost_graph[nx] + c {
                continue;
            }
            cost_graph[*y] = cost_graph[nx] + c;
            pq.push((-cost_graph[*y], *y));
        }
    }
}
