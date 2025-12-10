// kruskal mst

use std::collections::{HashMap};

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord,Clone,Copy)]
struct Edge {
    source : usize,
    dest : usize,
    cost : usize,
}

struct UnionFind{
    parent:Vec<usize>,
    rank:Vec<usize>,
}

impl UnionFind {

    fn new(n:usize) -> Self {
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
    
    fn kruskal_mst(&mut self,edges : Vec<Vec<Edge>>) -> Option<(i32,Vec<Edge>)> {
        
    }
    
}



fn main(){
    
    let num_cources:i32  = 5;
    let mut prereq:Vec<Vec<usize>> =  Vec::new();
    prereq.push(vec![0,1]);
    prereq.push(vec![1,2]);
    prereq.push(vec![2,3]);
    
}
