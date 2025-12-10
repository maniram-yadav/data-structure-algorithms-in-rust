// prims algo implemntation

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug,PartialEq,Eq)]
struct PrimeEdge {
    source : usize,
    dest : usize,
    cost : usize,
}

impl PrimeEdge {
    fn new(source : usize,dest:usize,cost:usize) -> Self {
        Self{
            source,
            dest,
            cost
        }
    }
    
}

impl Ord for PrimeEdge {

    fn cmp(&self,other : &Self) ->Ordering {
       other.cost.cmp(&self.cost)
    }    
}


impl PartialOrd for PrimeEdge {

    fn partial_cmp(&self,other : &Self) ->  Option<Ordering>  {
       Some(self.cmp(other))
    }    
}

fn prim_mst(edges : &mut Vec<PrimeEdge>, num_vertices : usize) -> Option<(usize,Vec<PrimeEdge>)> {
      let mut adj_list:HashMap<usize,Vec<(usize,usize)>> = HashMap::new();
      
      for edge in edges {
        adj_list.entry(edge.source).or_insert(Vec::new()).push((edge.dest,edge.cost));
      }
      
      let mut mst_edges = Vec::new();
      let mut min_heap = BinaryHeap::new();
      let mut visited = vec![false;num_vertices];
      
      visited[0] = true;
      
      if let Some(v) = adj_list.get(&0){
        // println!("{:?}",v);
          for edge in v {
              min_heap.push(PrimeEdge{source:0,dest:edge.0,cost:edge.1});
          }
      }
      let mut total = 0;
      while let Some(PrimeEdge{source,dest,cost}) = min_heap.pop(){
          if visited[dest] {
              continue;
          }
          visited[dest] = true;
          mst_edges.push(PrimeEdge{source,cost,dest});
          total += cost;
          if let Some(v) = adj_list.get(&dest){
                //  println!("{:?}",v);
                  for edge in v {
                    if !visited[edge.0] {
                      min_heap.push(PrimeEdge{source:dest,dest:edge.0,cost:edge.1});
                    }
                  }
            }
      
          
      }
      
      Some((total,mst_edges))
        
}

fn main(){
    
   let mut edges = vec![
        PrimeEdge::new(0, 1, 4),
        PrimeEdge::new(0, 2, 6),
        PrimeEdge::new(0, 3, 3),
        PrimeEdge::new(1, 2, 5),
        PrimeEdge::new(2, 3, 2),
        PrimeEdge::new(2, 4, 7),
        PrimeEdge::new(3, 4, 4),
    ];
    let num_vertices = 8;
    
    match prim_mst(&mut edges,num_vertices) {
    
    Some((cost,mst_edges)) => {
            println!("Total MST Cost : {}",cost);
            for edge  in &mst_edges {
                println!("{} {} -> {}",edge.source,edge.dest,edge.cost);
            }
            println!("Edges in Mst : {:?}",mst_edges);
        
    }
    None => {
        println!("Graph is disconnected, cannot form a single MST.");
    }
        
    }
    
}