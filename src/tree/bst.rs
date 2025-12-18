use std::cmp::Ordering;

#[derive(Debug)]
struct Node<T:Ord> {
    value : T,
    left : Option<Box<Node<T>>>,
    right : Option<Box<Node<T>>>,
}

impl<T:Ord> Node<T> {
    
    fn new(value : T) -> Self {
        Self{
            value,
            left:None,
            right:None,
        }
    }
}

pub struct BST<T:Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T:Ord> BST<T> {
    
        pub fn new() -> Self {
            Self{
                root:None,
            }
        }
        
        pub fn insert(&mut self,value : T) {
            self.root = Self::inert_recursive(self.root.take(),value);
        }
        
        fn inert_recursive(node : Option<Box<Node<T>>>, value : T) -> 
        Option<Box<Node<T>>>{
        
            match node {
                    None => Some(Box::new(Node::new(value))),
                    Some(mut c_node) => {
                        match value.cmp(&c_node.value)  {
                            Ordering::Less => {
                                c_node.left = Self::inert_recursive(c_node.left.take(),value);
                            },
                            Ordering::Greater => {
                                c_node.right = Self::inert_recursive(c_node.right.take(),value);
                            },
                            Ordering::Equal => {
                                
                            },
                        }
                        Some(c_node)
                    }
                
            }
            
        }
        
        pub fn find(&self,value : &T) -> bool {
            let mut current = &self.root;
            while let Some(node) = current {
                match value.cmp(&node.value){
                    Ordering::Less => {
                       current = &node.left; 
                    },
                    Ordering::Greater => {
                       current = &node.right; 
                    },
                    Ordering::Equal => {
                      return true; 
                    }
                }
            }
            
            false
        }
        
}

fn main(){
    let mut bst = BST::<i32>::new();
    bst.insert(3);
    bst.insert(2);
    bst.insert(1);
    bst.insert(8);
    println!("Found {} : {}",1,bst.find(&1));
    println!("Found {} : {}",3,bst.find(&3));
    println!("Found {} : {}",5,bst.find(&5));
    println!("Found {} : {}",9,bst.find(&8));
}
