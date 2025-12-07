#[allow(unused)]

pub struct UnionFind {
    parent:Vec<usize>,
    size:Vec<usize>,
    num_sets:usize,
}

impl UnionFind {
    
    fn new( n: usize) -> Self {
        Self{
            parent : (0..n).collect(),
            size : vec![1;n],
            num_sets:n,
        }
    }
    
    fn find(&mut self,i : usize) -> usize {
        
        if self.parent[i] == i {
            i
        } else {
            let parent = self.find(self.parent[i]);
            self.parent[i] = parent;
            parent
        }
    }
    
    fn union(&mut self,i : usize,j:usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);
        
        if root_i != root_j {
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
            self.num_sets -= 1;
            true
        } else {
            false
        }
        
    }
    
    pub fn count(&self) -> usize {
        self.num_sets
    }
    
    pub fn is_same_set(&mut self,i : usize,j:usize) -> bool {
        self.find(i) == self.find(j)
    }
    
    }
 
 #[cfg(test)]
 mod union_tests {
    use super::*;
     #[test]
     fn test_union_find() {
         let mut uf = UnionFind::new(5);

        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.find(1), 1);
        assert!(!uf.is_same_set(0, 1));

        uf.union(0, 1);
        assert!(uf.is_same_set(0, 1));
        assert_eq!(uf.find(0), uf.find(1));

        uf.union(2, 3);
        assert!(uf.is_same_set(2, 3));
        assert!(!uf.is_same_set(0, 2));

        uf.union(1, 3);
        assert!(uf.is_same_set(0, 3));
        assert!(uf.is_same_set(1, 2));
        assert_eq!(uf.find(0), uf.find(2));
        assert_eq!(uf.find(0), uf.find(3));
        assert!(!uf.is_same_set(0, 4));
     }
 }
     
    fn main(){
        
    }