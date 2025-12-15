// 2276. Count Integers in Intervals
// https://leetcode.com/problems/count-integers-in-intervals/description/

use std::collections::{BTreeMap};
use std::ops::Bound::{self,Included,Excluded,Unbounded};
use std::cmp;

struct CountIntervals {
    count : i32,
    ele : BTreeMap<i32,i32>,
}

impl CountIntervals {

    fn new() -> Self {
        Self{
            count:0,
            ele : BTreeMap::<i32,i32>::new(),
        }
    }
    
    fn add(&mut self, left: i32, right: i32) {
        let mut map = &mut self.ele;
        let mut min = left;
        let mut max = right;
        
        let mut total = 0;
        let mut lowest_lesser_entry = map
            .range((Unbounded, Excluded(min)))
            .next_back();
        // if lowest_lesser_entry == None {
        //     lowest_lesser_entry = Some((&min,&max));
        // }
        
        let mut left1  = left;
        let mut right1 = right ;
        
        if let Some((key , value)) = lowest_lesser_entry {
            if *value >= min {
                left1 = *key;
                right1 = cmp::max(*value,max);
                // total += (*value-*key)
            }
        }
        println!(" left : {} right : {} Lowest Min : {:?} ",left,right,lowest_lesser_entry);
        while let Some((&key,value)) = map.range_mut((Included(left1),Included(right1))).next() {
            min = cmp::min(min,key);
            max = cmp::max(max,*value);
            total += *value-key+1;
            println!("Key : {} {}  {} {} {}",key,value.clone(),min,max,total );
            map.remove(&key);
            
        }
        let range = (max-min+1)-total;
        map.entry(min).or_insert(max);
        self.count += range;
        
    }
    
    fn count(&self) -> i32 {
        
        self.count
    }
}

