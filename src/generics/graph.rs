
use std::collections::HashMap;
use std::hash::Hash;
// rust generics example


struct Graph<T> {
  adjency_list: HashMap<T,Vec<T>>,
}


trait GraphOps<T> {
  fn add_edge(&mut self,from:T,to:T) ;
  fn get_neighbour(&self,node:&T) -> Option<&Vec<T>>  ;
}


impl<T> Graph<T>
  where T : Eq + Hash + Clone
{
  fn new() -> Self {
      Self{
        adjency_list:HashMap::new()
      }
  }
}

impl<T> GraphOps<T> for Graph<T> 
  where T : Eq + Hash + Clone {
    
    fn add_edge(&mut self,from : T, to : T) {
     
      self.adjency_list.entry(from.clone())
            .or_insert_with(Vec::new)
            .push(to.clone());
      
      self.adjency_list.entry(to)
           .or_insert_with(Vec::new);
      
            
            
    }
    
    fn get_neighbour(&self,node : &T) -> Option<&Vec<T>> {
      
         self.adjency_list.get(node)
    }
  }

fn main() {
  let mut graph : Graph<i32> = Graph::new(); 
  graph.add_edge(1,2);
  graph.add_edge(2,3);
  
  graph.add_edge(2,8);
     println!("Neighbour of {} : {:?}",9,graph.get_neighbour(&9));
}

