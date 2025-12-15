// 56  merge interval
https://leetcode.com/problems/merge-intervals/description/

use std::cmp;

fn merge_interval(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>{
    
    let mut intervals = intervals.clone(); 
    intervals.sort_by_key(|nested| nested[1]);
   
    let n = intervals.len();
    if n==1 {
        return intervals;
    }
    
    let mut low = intervals[0][0].clone();
    let mut high = intervals[0][1].clone();
    
    let mut result: Vec<Vec<i32>> = Vec::new();
    
    for i in 1..n {
        let curr = &intervals[i];
        let prev = &intervals[i-1];
        
        if curr[0] <= high {
            high = curr[1];
            low = cmp::min(prev[0],curr[0]);
        } else {
            result.push(vec![low,high]);
            low = curr[0];
            high = curr[1];
        }
    }
    result.push(vec![low,high]);
    return result;
    
}

fn main() {
    
    let intervals:Vec<Vec<i32>> = vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]]; 
    let result = merge_interval(intervals);
    println!("{:?}",result);
}
