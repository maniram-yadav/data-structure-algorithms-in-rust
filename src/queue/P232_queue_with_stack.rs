// https://leetcode.com/problems/implement-queue-using-stacks/description/

struct MyQueue {
    input_stack:Vec<i32>,
    output_stack:Vec<i32>,
}

impl MyQueue {

    fn new() -> Self {
        Self{
            input_stack : Vec::new(),
            output_stack : Vec::new(),
        }
    }
    
    pub fn push(&mut self,v : i32) {
        self.input_stack.push(v);
    }

    pub fn pop(&mut self) -> i32{
        self.transfer_if_needed();
        self.output_stack.pop().unwrap()
    }

    pub fn peek(&mut self) -> i32{
        self.transfer_if_needed();
        *self.output_stack.last().unwrap()
    }

   pub fn empty(&self) -> bool{
        self.input_stack.is_empty() &&  self.output_stack.is_empty() 
    }
    
    pub fn transfer_if_needed(&mut self) {
        if self.output_stack.is_empty() {
            while let Some(value) = self.input_stack.pop() {
                self.output_stack.push(value);
            }
        }
    }
    
}

fn main() {
    
     let mut queue = MyQueue::new();
     queue.push(12);
     queue.push(18);
     queue.push(70);
     println!("{}",queue.peek());
     println!("{}",queue.pop());
     println!("{}",queue.empty());
     println!("{}",queue.peek());
     
     println!("{}",queue.pop());
     println!("{}",queue.empty());
     println!("{}",queue.peek());
     
     println!("{}",queue.pop());
     println!("{}",queue.empty());
}
