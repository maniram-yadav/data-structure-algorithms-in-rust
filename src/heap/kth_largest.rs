use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn find_k_largest(arr : &[i32],n :usize) -> Option<i32> {
    
    if n==0 || n > arr.len() {
        return None;
    } 
    
    let mut min_heap:BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    for &v in arr.iter() {
        if min_heap.len() < n {
            min_heap.push(Reverse(v));
        } else if v > min_heap.peek().unwrap().0  {
            min_heap.pop();
            min_heap.push(Reverse(v));
        }   
    }
    
    return  min_heap.pop().map(|rev_num| rev_num.0);
    }

fn main() {

    let arr = vec![3,2,6,1,9]; 
    let val = find_k_largest(&arr,3);
    let v = match val{ Some(value) => value, None => -1 };
    println!(" {} ",v);
    
}

mod tests {
    use super::*;
    
    #[test]
    fn test_find_k_largest() {
        let arr = vec![3,2,1,5,6,4];
        assert_eq!(find_k_largest(&arr,2), Some(5));
        assert_eq!(find_k_largest(&arr,4), Some(3));
        assert_eq!(find_k_largest(&arr,0), None);
        assert_eq!(find_k_largest(&arr,10), None);
    }
}