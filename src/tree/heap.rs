
use std::{cmp::Ord};

struct Heap<T>{
    items:Vec<T>,
    comparator : fn(&T,&T)->bool,
}


impl<T> Heap<T> {
    pub fn new(comparator : fn (&T,&T)->bool)->Self {
        Self {
            items:vec![],
            comparator,
        }
    }

    pub fn from_vec(items:Vec<T>, comparator : fn (&T,&T)->bool) -> Self {
       let mut heap =  Self {
            items,
            comparator,
        };
        heap.heapify();
        heap
    }

    
    pub  fn add(&mut self,value : T){
        self.items.push(value);
        self.heapify_up(self.len()-1);
    }   

    
    pub  fn pop(&mut self) -> Option<T>{
        
        if self.is_empty(){
            return    None;
        } 
        
        let next = Some(self.items.swap_remove(0));
        if !self.is_empty() {
                self.heapify_down(0);
        }
        next
    }   

    pub  fn heapify(&mut self){
            let last_parent_index = (self.len()/2).wrapping_sub(1);
            for index in (0..=last_parent_index).rev() {
                    self.heapify_down(index);
            }

    }   
    
    
     fn heapify_up(&mut self,mut index : usize){
            println!("Inside heapify down");
            while let  Some(parent_index) = self.parent_index(index){
                if (self.comparator)(&self.items[index],&self.items[parent_index]){
                        self.items.swap(parent_index,index);
                        index = parent_index;
                } else {
                    break;
                }
            }       
    }   

    
     fn heapify_down(&mut self,mut index : usize){
        println!("Inside heapify down");
        while self.is_children_present(index) {

            let current_index = {

                    if self.right_child(index) >= self.len() {
                        self.left_child(index)
                    } else{
                        let leftindex = self.left_child(index);
                        let rightindex = self.right_child(index);
                        if (self.comparator)(&self.items[leftindex],&self.items[rightindex]){
                                rightindex
                        } else {
                                leftindex
                        }
                    }
                
            };

            if (self.comparator)(&self.items[current_index],&self.items[index]){
                self.items.swap(current_index,index);
                index = current_index;
            } else {
                break;
            }

        }
    }   


    pub fn parent_index(&self,index : usize) -> Option<usize> {
        if index>0 {
                Some((index-1)/2)
        } else {
                None
        }
    }
    
    pub fn peek(&self) -> Option<&T> {
        if !self.is_empty(){
            return  Some(&self.items[0]);
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.len()==0
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
    fn is_children_present(&self,index : usize) -> bool {
        self.left_child(index)<self.len()
    }
    fn left_child(&self,index : usize) -> usize {
        index*2+1
    }
    
    fn right_child(&self,index : usize) -> usize {
        self.left_child(index)+1
    }
}

impl<T> Heap<T> where T:Ord {
    pub fn new_min() ->Heap<T>{
        Self::new(|a,b| a<b)
    }
    pub fn new_max() ->Heap<T>{
        Self::new(|a,b| a>b)
    }
    pub fn from_vec_min(items:Vec<T>) ->Heap<T>{
        Self::from_vec(items,|a,b| a<b)
    }
    
    pub fn from_vec_max(items:Vec<T>) ->Heap<T>{
        Self::from_vec(items,|a,b| a>b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap(){
        let mut heap:Heap<i16> = Heap::new_max();
        assert_eq!(heap.pop(),None);

    }

}