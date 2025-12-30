use std::collections::{HashMap, HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        
        if words.len() ==0{
            return "".to_string();
        }
        let mut adj:HashMap<char,HashSet<char>> = HashMap::new();
        let mut degrees : HashMap<char,i32> =HashMap::new();
        
        for word in &words {
            for c in word.chars(){
                degrees.entry(c).or_insert(0);        
            }
        }
        let n = words.len();
        for i in 1..n {
            let word1 = &words[i-1];
            let word2 = &words[i];
            let min_len = std::cmp::min(word1.len(),word2.len());
            if word1.len()>word2.len() && word1.starts_with(word2) {
                return "".to_string();
            }
            
            for j in 0..min_len {
                let char1 = word1.chars().nth(j).unwrap();
                let char2 = word2.chars().nth(j).unwrap();
                
                if char1 != char2 {
                    
                    if adj.entry(char1).or_insert(HashSet::new()).insert(char2) {
                        *degrees.entry(char2).or_insert(0) += 1;
                    }
                    break;
                }
            }
            
        }
        
        let mut result = String::new();
        let mut queue:VecDeque<char> = VecDeque::new();
        
        for (&c,&degree) in &degrees {
            if degree==0 {
                queue.push_back(c);
            }
        }
        
        while let Some(ch) = queue.pop_front(){
            result.push(ch);
            if let Some(neighbours) = adj.get(&ch) {
                for &neighbour in neighbours {
                    if let Some(degree)= degrees.get_mut(&neighbour) {
                       *degree -= 1;
                        if *degree== 0 {
                            queue.push_back(neighbour);
                        }   
                    }
                   
                } 
            }
        }
        
        if result.len()==degrees.len() {
            result
        } else {
            return "".to_string();    
        }
        
    }
  
}

  fn main() {
    // Example 1
    let words1 = vec!["wrt".to_string(), "wrf".to_string(), "er".to_string(), "ett".to_string(), "rftt".to_string()];
    println!("Input: {:?}", words1);
    println!("Output: \"{}\"", Solution::alien_order(words1)); // Expected: "wertf"

    println!("---");

    // Example 2
    let words2 = vec!["z".to_string(), "x".to_string()];
    println!("Input: {:?}", words2);
    println!("Output: \"{}\"", Solution::alien_order(words2)); // Expected: "zx"

    println!("---");

    // Example 3 (Cycle case)
    let words3 = vec!["z".to_string(), "x".to_string(), "z".to_string()];
    println!("Input: {:?}", words3);
    println!("Output: \"{}\"", Solution::alien_order(words3)); // Expected: ""

    println!("---");
    
    // Example 4 (Prefix case)
    let words4 = vec!["abc".to_string(), "ab".to_string()];
    println!("Input: {:?}", words4);
    println!("Output: \"{}\"", Solution::alien_order(words4)); // Expected: ""
}