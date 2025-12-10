

// kruskal mst

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord,Clone,Copy)]
struct Edge {
    source : usize,
    dest : usize,
    cost : usize,
}

impl Edge {

    fn new(source:usize,dest:usize,cost:usize) -> Self {
        Self{
            source,
            dest,
            cost,
        }
    }    
}

struct UnionFind{
    parent:Vec<usize>,
    rank:Vec<usize>,
}

impl UnionFind {

   pub fn new(n:usize) -> Self {
            Self{
                parent:(0..n).collect(),
                rank:vec![0;n],
            }
    }
    
    fn find(&mut self,i : usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            self.parent[i] = self.find(self.parent[i]);
            self.parent[i]
        }
    } 
    fn union(&mut self,i : usize,j : usize)  {
        
        let p1 = self.find(i);
        let p2 = self.find(j);
        if p1 != p2 {
            if self.rank[p1] > self.rank[p2] {
                self.parent[p2] = p1;
                self.rank[p1] += self.rank[p2];
            } else {
               self.parent[p1] = p2;
                self.rank[p2] += self.rank[p1];
                
            }
        }
    }
    
    
}


fn kruskal_mst(edges : &mut Vec<Edge>, num_vertices : usize) -> Option<(usize,Vec<Edge>)> {
      
        edges.sort_by_key(|e| e.cost);
        let mut uf = UnionFind::new(num_vertices);
        let mut mst_edges = Vec::new();
        let mut total_cost = 0;
        let mut edge_count = 0;
        
        for &edge in edges.iter() {
            uf.union(edge.source,edge.dest);
            mst_edges.push(edge);
            total_cost += edge.cost;
            edge_count += 1;
            if edge_count == num_vertices-1 {
                break;
            }
        }
        if edge_count == num_vertices-1 {
            Some((total_cost,mst_edges))
        } else {
            None
        }
        
}

fn main(){
    
   let mut edges = vec![
        Edge::new(0, 1, 4),
        Edge::new(0, 2, 6),
        Edge::new(0, 3, 3),
        Edge::new(1, 2, 5),
        Edge::new(2, 3, 2),
        Edge::new(2, 4, 7),
        Edge::new(3, 4, 4),
    ];
    let num_vertices = 5;
    
    match kruskal_mst(&mut edges,num_vertices) {
    
    Some((cost,mst_edges)) => {
            println!("Total MST Cost : {}",cost);
            println!("Edges in Mst : {:?}",mst_edges);
        
    }
    None => {
        println!("Graph is disconnected, cannot form a single MST.");
    }
        
    }
    
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kruskal_mst(){
      
   let mut edges = vec![
        Edge::new(0, 1, 4),
        Edge::new(0, 2, 6),
        Edge::new(0, 3, 3),
        Edge::new(1, 2, 5),
        Edge::new(2, 3, 2),
        Edge::new(2, 4, 7),
        Edge::new(3, 4, 4),
    ];
    let num_vertices = 5;
    
    match kruskal_mst(&mut edges,num_vertices) {
    
    Some((cost,mst_edges)) => {
            println!("Total MST Cost : {}",cost);
            println!("Edges in Mst : {:?}",mst_edges);
        
    }
    None => {
        println!("Graph is disconnected, cannot form a single MST.");
    }
        
    }

    }

}