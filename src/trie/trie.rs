// trie implemntation

use std::collections::{ HashMap};

#[derive(Default)]
struct TrieNode{
     children : HashMap<char,TrieNode>,
     is_end_of_word: bool,
}

struct Trie{
    root : TrieNode,
}

impl Trie {


    fn new() -> Self{
        Self{
            root:TrieNode::default(),
        }
    }
    

    fn insert(&mut self,word:&str)  {
        let mut current_node = &mut self.root;
    
        for ch in word.chars() {
            current_node = current_node.children.entry(ch)
                    .or_insert_with(TrieNode::default);
        }
        current_node.is_end_of_word = true;
    }
    
    
    fn search(&self,word:&str) ->bool{
        let mut current_node = &self.root;
    
        for ch in word.chars() {
           match  current_node.children.get(&ch){
               Some(v) => {
                   current_node = v;
               } ,
               None => {
                   return false;
               }
           }
            
        }
       return current_node.is_end_of_word;
    }
    
     fn starts_with(&self,word:&str) ->bool{
        let mut current_node = &self.root;
    
        for ch in word.chars() {
           match  current_node.children.get(&ch){
               Some(v) => {
                   current_node = v;
               } ,
               None => {
                   return false;
               }
           }
            
        }
       return true;
    }
}

fn main(){
            let mut trie = Trie::new();
            trie.insert("hello");
            println!("Serach {} : Found : {}","hello",trie.search("hello"));
            println!("Serach {} : Found : {}","helalo",trie.search("helalo"));
            println!("Starts {} : Found : {}","hel",trie.starts_with("hel"));
            
    
}