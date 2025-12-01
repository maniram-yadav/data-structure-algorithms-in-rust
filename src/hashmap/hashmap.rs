// custom hashmap implementation 

use std::hash::{Hash,Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Debug,Copy,Clone)]
struct Entry<K,V>{
    key : K,
    value : V,
}

pub struct HashMap<K,V> {
    entries : Vec<Vec<Entry<K,V>>>,
    size : usize
}

impl<K,V> HashMap<K,V> where K : Hash + Eq+Clone,
            V:Clone {
    
    pub fn new(buckets:usize) -> Self {
    
            Self {
                entries : vec![Vec::<Entry<K, V>>::new();buckets],
                size:0,
            }
    }
    
    pub fn insert(&mut self , k:K,v:V){
        let key = self.get_bucket_index(&k);
        let bucket = &mut self.entries[key];
        for entry in bucket.iter_mut() {
            if entry.key == k {
                entry.value = v;
                return;
            }
        }
        bucket.push(Entry{key:k,value:v});
        self.size+=1;
    }
    
    pub fn get(&self , k:&K) -> Option<&V> {
        
        let key = self.get_bucket_index(k);
        let bucket = &self.entries[key];
        
        for entry in bucket.iter() {
            if entry.key == *k {
                   return Some(&entry.value);
            }
        }
        None
    }
    
     pub fn remove(&mut self , k:&K) -> Option<V> {
        
        let key = self.get_bucket_index(k);
        let mut bucket = &mut self.entries[key];
        let mut index = 0;
        for entry in bucket.iter() {
            if entry.key == *k {
                  break;
            }
            index+=1;
        }
        if index < bucket.len() {
            self.size -= 1;
            return Some(bucket.remove(index).value);
        }
        None
        
        
    }
    
    fn get_bucket_index(&self,k:&K) -> usize {
            let mut hasher = DefaultHasher::new();
            k.hash(&mut hasher);
            (hasher.finish() % self.entries.len() as u64) as usize
    }
    
    fn size(&self) -> usize {
        self.size
    } 
    
}

fn main(){
    
    let mut map = HashMap::<i32,i32>::new(10);
    map.insert(10,20);
    map.insert(12,90);
    match map.get(&10)  {
        Some(ref value) => println!("{} ",value),
        None => println!("No value found")
    }
    println!("map Size {} ",map.size());
     match map.remove(&30)  {
        Some(value) => println!("removed {} ",value),
        None => println!("No value found")
    }
    
    println!("map Size {} ",map.size());
}