
mod array;
mod tree;
mod concurrent;

use tree::binarytree::{BinaryTree,BinaryNode};
use concurrent::concurrent::*;


#[allow(dead_code)]
fn main(){
    let  arr = vec!{91,12,3,78};

    let mut clone = arr.clone();
    array::bubble::sort(&mut clone);
    println!("Sorted array {:?}",clone);
    
    
    clone = arr.clone();
    array::selection::sort(&mut clone);
    println!("Sorted array {:?}",clone);

    let mut btree : &mut BinaryTree<i32> =  &mut BinaryTree::new();
    println!("{}",btree.root().is_filled());
    let mut root :  &mut BinaryNode<i32> = btree.root_mut();
    let _left1 = root.append_left(14).ok();    
    let _right1 = root.append_right(19).ok();

    // root.print_value();
    ConcurrentTest::new().test_dining_philosopher();

}