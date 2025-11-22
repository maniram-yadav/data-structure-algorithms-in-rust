// FIX the errors with least changes
// DON'T remove any code line
use std::ptr;
use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;

#[derive(Debug)]
struct Node<T> {
    data : T,
    next : *mut Node<T>,
}

impl<T: Display> Display for  Node<T> {
   fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.data)
    }
}

impl<T:Debug> Node<T>  {

    fn print(&self) {
        if !self.next.is_null(){
            unsafe {
            let node = Box::from_raw(self.next);
            node.print();
            }
        }
        print!(" {:?} ",self.data);
    }
    
}

struct LinkedList<T> {
    head : *mut Node<T>,
    size:usize
}

impl<T:Debug>  LinkedList<T> {
    fn new() -> Self {
        Self{
            head:ptr::null_mut(),
            size:0
        }
    }
    
    fn push_front(&mut self,data:T){
        let  new_node = Box::new(Node{data,
                next:self.head
                } );
        self.head = Box::into_raw(new_node);
        self.size += 1;
        
    }
    
    fn pop(&mut self) -> Option<T>{
        if self.head.is_null()  {
            return None;
        }
        self.size -= 1;
        unsafe {
            let old_ptr = self.head;
            self.head = (*old_ptr).next;
            let old_node = Box::from_raw(old_ptr);
            Some(old_node.data)
        }
    }
    fn len(&self) -> usize{ 
        self.size
    }
    
    fn print(&self) {
        if !self.head.is_null(){
            unsafe{
            let head = Box::from_raw(self.head);
            let node = head.as_ref();
            node.print();
            }
        }
    }
     fn print2(&self) {
        let mut current : *const Node<T> = self.head;
        print!("[ ");
        while !current.is_null(){
            unsafe{
                print!("{:?} ", (*current).data);
                current = (*current).next;
                if !current.is_null(){
                    print!(", ");
                }
            }
        }
        println!(" ]");
        
    }
}

fn main() {
     let mut list = LinkedList::<i32>::new();
     list.push_front(10);
     list.push_front(20);
     list.push_front(30);
     list.push_front(40);
     println!("{}",list.len());
     list.print2();
     list.print2();
     println!();
     list.push_front(40);
     println!("{}",list.len());
     //consumes all the element
     list.print();
}