

use std::ptr;
use std::mem;

type Link<T> = *mut Node<T>;

struct Node<T> {
    v : T,
    prev : Link<T>,
    next : Link<T>,
}

struct DLL<T> {
    head : Link<T>,
    tail : Link<T>,
    size : u32,
}

impl<T:std::fmt::Debug> DLL<T>{
    fn new() -> Self {
        Self {
            head:ptr::null_mut(),
            tail:ptr::null_mut(),
            size:0,
        }
    }
    
    fn push_front(&mut self,v:T){
        let node = Box::into_raw(
                    Box::new(Node{
                        v,
                        next:self.head,
                        prev:ptr::null_mut(),
                    }));
        unsafe {
            if !self.head.is_null() {
                (*self.head).prev = node;
            } else {
                self.tail = node;
            }
            self.head = node;
            self.size += 1;
        }
    }
    
     
    fn pop_front(&mut self){
            unsafe {
            if self.head.is_null(){
                
            } else if !self.head.is_null() && self.head==self.tail {
                 self.head = ptr::null_mut();
                 self.tail = ptr::null_mut();
                 self.size -= 1;
            } else {
                self.head = (*self.head).next;
                (*self.head).prev = ptr::null_mut();
                self.size -= 1;
                
            } 
            
        }
    }
    
    fn len(&self) -> u32{
        self.size
    }
    
    fn print(&self) {
        let mut start : Link<T> = self.head;
        while !start.is_null() {
          
          unsafe {
            let node = Box::from_raw(start);
            let next = node.next;
            println!("{:?} ", node.v);
            start = next;
            }
        }
    }
    
    
}

fn main(){
    let mut dll : DLL<i32> = DLL::new();
    dll.push_front(5);
    dll.push_front(11);
    dll.push_front(14);
    dll.print();
    dll.pop_front();
    dll.print();
    dll.pop_front();
    println!("Size : {} ",dll.len());
    dll.pop_front();
    dll.pop_front();
    println!("Size : {} ",dll.len());
    dll.print();
    
}
