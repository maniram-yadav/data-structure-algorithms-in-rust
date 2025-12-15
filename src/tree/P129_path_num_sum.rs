// 129. Sum Root to Leaf Numbers


// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut total = 0;

        Self::sum(0,&root,&mut total);
        total
    }
    pub fn sum(current:i32,root: &Option<Rc<RefCell<TreeNode>>>,
     total:&mut i32)  {

        if let Some(node) = root {

            let node_ref = node.borrow();
            let next = current*10 + node_ref.val;
            if node_ref.left.is_none() && node_ref.right.is_none() {
                *total += next;
               return ;
            }
            Self::sum(next,&node_ref.left,total);
            Self::sum(next,&node_ref.right,total);
        }   
    }
    
}