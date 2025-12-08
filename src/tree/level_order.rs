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
    
    
    pub fn level_order(root : Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        
        let mut result:Vec<Vec<i32>> = Vec::new();
        let mut queue : VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        
        if let Some(node) = root {
            queue.push_back(node);
        }
        
        while !queue.is_empty() {
            let mut queue_size = queue.len();
            let mut current_level:Vec<i32> = Vec::new();
            
            while queue_size>0 {
                queue_size -= 1;
                if let Some(node_rc) = queue.pop_front() {
                    let current = node_rc.borrow();
                    current_level.push(current.val);
                    if let Some(left_child) = current.left.clone() {
                        queue.push_back(left_child);
                    }
                    if let Some(right_child) = current.right.clone() {
                        queue.push_back(right_child);
                    }
                }
            }
            result.push(current_level);
        }
        
        result
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
    let result = TreeNode::level_order(Some(root3));
    println!("Level Order {:?}",result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_order(){
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
    let result = TreeNode::level_order(Some(root3));
    assert_eq!(result, vec![vec![3],vec![9,20],vec![15,7]]);
    println!("Level Order {:?}",result);

    }

}