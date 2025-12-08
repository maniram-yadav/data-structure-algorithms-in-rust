use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

pub struct TreeNode{
    pub val : i32,
    pub left : Option<Rc<RefCell<TreeNode>>>,
    pub right : Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {

    #[inline]
    pub fn new(val:i32) -> Self {
        Self{
            val,
            left : None,
            right : None,
        }
    }
   
}

pub fn inorder_traversal(root:Option<Rc<RefCell<TreeNode>>>, result : &mut Vec<i32>) {
    
    if let Some(node) = root {
        let n_borrow = node.borrow();
        inorder_traversal(n_borrow.left.clone(),result);
        result.push(n_borrow.val);
        inorder_traversal(n_borrow.right.clone(),result);
        
    }
}
 
fn main(){

  // Construct a sample tree:
    //       3
    //      / \
    //     9  20
    //       /  \
    //      15   7
    
    let left15 = Rc::new(RefCell::new(TreeNode::new(15)));
    let right7 = Rc::new(RefCell::new(TreeNode::new(7)));
    let right20 = Rc::new(RefCell::new(TreeNode{
            val : 20,
            left :Some(left15),
            right :Some(right7),
        }));
    let left9 = Rc::new(RefCell::new(TreeNode::new(9)));
    let root3 = Rc::new(RefCell::new(TreeNode{
            val : 3,
            left :Some(left9),
            right :Some(right20),
        }));
    let mut result:Vec<i32> = Vec::new();
    inorder_traversal(Some(root3),&mut result);
    println!("InOrder  {:?}",result);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inorder(){
       let left15 = Rc::new(RefCell::new(TreeNode::new(15)));
    let right7 = Rc::new(RefCell::new(TreeNode::new(7)));
    let right20 = Rc::new(RefCell::new(TreeNode{
            val : 20,
            left :Some(left15),
            right :Some(right7),
        }));
    let left9 = Rc::new(RefCell::new(TreeNode::new(9)));
    let root3 = Rc::new(RefCell::new(TreeNode{
            val : 3,
            left :Some(left9),
            right :Some(right20),
        }));
    let mut result:Vec<i32> = Vec::new();
    inorder_traversal(Some(root3),&mut result);
    assert_eq!(result, vec![9,3,15,20,7]);
    println!("InOrder  {:?}",result);
    }

}