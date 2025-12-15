use std::cmp::max;
use std::fmt::Debug;

pub struct SegmentTree<T,F> where 
        T : Clone+Copy+Debug,
        F : Fn(T,T) -> T {
            n : usize,
            tree : Vec<T>,
            def : T,
            op : F
        }
        
impl<T,F> SegmentTree<T,F> where
    T: Clone + Copy + Debug,
    F: Fn(T, T) -> T {
       
        pub fn  new(arr : &[T],def : T, op : F) -> Self {
            
            let n = arr.len();
            let mut tree = vec![def;2*n];
            
            for i in 0..n {
                tree[n+i] = arr[i];
            }
            
            for i in (1..n).rev() {
                tree[i] = op(tree[2*i],tree[2*i+1]);
            }
            Self{
                n,
                tree,
                def,
                op
            }
            
        } 
        
        pub fn update(&mut self,mut i:usize,val:T){
            i += self.n;
            self.tree[i]=val;
            while i>=1 {
                i/=2;
                self.tree[i] = (self.op)(self.tree[2*i],self.tree[2*i+1]);
            }
            
        }
        
        pub fn query(&self,mut l:usize, mut r:usize) -> T {
            l += self.n-1;
            r += self.n;
            let mut left = self.def;
            let mut right = self.def;
            
            while l<r {
                if l%2==1 {
                    left = (self.op)(left,self.tree[l]);
                    l += 1;
                }    
                if r%2 == 1{
                    r -= 1;
                    right = (self.op)(right,self.tree[r]);
                }
                l/=2;
                r/=2;
            }
            
            (self.op)(left,right)
        }
        
        pub fn print(&self){
            let tree = &self.tree;
            let n = 2*self.n;
            for i in 0..n {
                println!("({:?},{:?}),  ",i,tree[i]);
            }
        }
    } 
    
    
fn main() {

        let data = vec![1, 3, 8, 2, 9, 4, 6, 5];
        let mut seg_tree = SegmentTree::new(&data,0,|a,b| a+b);
        let (left,right) = (3,5);
        let val = seg_tree.query(left,right);
        seg_tree.print();
        println!(" {} {} : {}" ,left,right,val);
  
}
