use std::collections::VecDeque;

struct Solution;

impl Solution {
    /// Finds the cheapest price from src to dst with at most k stops.
    /// Uses a modified BFS approach.
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        
        let src = src as usize;
        let dst = dst as usize;
        let n = flights.len() ;
        let mut adj = vec![Vec::<(usize,i32)>::new();n];
        for i in 0..n {
            let source = flights[i][0] as usize;
            let dest = flights[i][1] as usize;
            let cost = flights[i][2] ;
            adj[source].push((dest,cost));
        }
        
        let mut dist = vec![i32::MAX;n];
        let mut queue = VecDeque::<(usize,i32)>::new();
        
        dist[src as usize]=0;
        queue.push_back((src,0));
        for _ in 0..=k {
            let n = queue.len();
            for j in 0..n {
             let (source,cost) = queue.pop_front().unwrap();
                for &(dest,price )in &adj[source] {
                       if dist[dest]> cost + price{
                            dist[dest] = cost + price;
                            queue.push_back((dest,dist[dest]));
                        }
                }
              }
            
        }
        if dist[dst]==i32::MAX{
            -1
        } else {
            
            dist[dst as usize]
        }
    }
}

fn main() {
    // Example 1
    let n1 = 4;
    let flights1 = vec![vec![0, 1, 100], vec![1, 2, 100], vec![2, 0, 100], vec![1, 3, 600], vec![2, 3, 200]];
    let src1 = 0;
    let dst1 = 3;
    let k1 = 1;
    println!("Example 1 Result: {}", Solution::find_cheapest_price(n1, flights1, src1, dst1, k1)); // Expected: 700

    // Example 2
    let n2 = 3;
    let flights2 = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
    let src2 = 0;
    let dst2 = 2;
    let k2 = 1;
    println!("Example 2 Result: {}", Solution::find_cheapest_price(n2, flights2, src2, dst2, k2)); // Expected: 200
}
