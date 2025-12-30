struct Solution;

impl Solution {
    
    fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        
        let n = graph.len();
        let mut colors = vec![0;n];
                
        for i in 0..n {
                let mut stack = vec![i];
                if colors[i]==0{
                    colors[i]=1;
                }
                while let Some(u)=stack.pop(){
                    for &v in graph[u].iter(){
                        let v = v as usize;
                        if colors[v]==0{
                            colors[v] = -colors[u];
                            stack.push(u);
                        } else if colors[v]==colors[u]{
                            return false;
                        } 
                        
                    }
                }
        }
        true
    }
    
}

fn main() {
    // Example 1: Not bipartite (contains an odd-length cycle)
    let graph1 = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
    println!("Graph 1 is bipartite: {}", Solution::is_bipartite(graph1)); // Expected: false

    // Example 2: Bipartite
    let graph2 = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
    println!("Graph 2 is bipartite: {}", Solution::is_bipartite(graph2)); // Expected: true

    // Example 3: Disconnected graph, still bipartite
    let graph3 = vec![vec![1], vec![0], vec![3], vec![2]];
    println!("Graph 3 is bipartite: {}", Solution::is_bipartite(graph3)); // Expected: true
}