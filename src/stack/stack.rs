// stack custom implementation


type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug,Clone)]
struct Node<T> {
    elem : T,
    next : Link<T>,
}

struct ListStack<T> {
    head : Link<T>,
    size:usize,
}

impl<T:Clone> ListStack<T> {
    fn new() -> Self {
        Self {
            head : None,
            size:0,
        }
    }
    
    fn push(&mut self,v:T) {
        let mut new_node = Node{ elem : v,next:None };
        new_node.next = self.head.take();
        self.head = Some(Box::new(new_node)) ;
        self.size+=1;
    }
    
    fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        
        self.size-=1;
        
        //first way
        // self.head.take().map(|mut node| {
        //     self.head = node.next.take();
        //     node.elem
        // })
        
        
        // scond way
        
        let popped_node  = self.head.take();
        match popped_node {
            Some(mut node_box) => {
                self.head = node_box.next.take();
                let element = node_box.elem;
                Some(element)
            },
            None=> {
                None    
            },
        }
        
    }
    
    fn len(&self) -> usize {
        self.size
    }
    
    
}

fn main() {
    let mut stack = ListStack::<i32>::new();
    stack.push(12);
    stack.push(14);
    stack.push(17);
    let top = stack.pop();
    println!("{:?} ",top);
    println!("{:?} ",stack.len());
    println!("{:?} ",stack.pop());
    println!("{:?} ",stack.pop());
    println!("{:?} ",stack.len());
    println!("{:?} ",stack.pop());
}