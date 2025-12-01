// queue custom implemntation


struct Node<T> {
    elem : T,
    next : Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Queue<T> {
    head: Link<T>,
    tail:Option<*mut Node<T>>,
    size: usize,
}

impl<T>  Queue<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail : None,
            size : 0,
        }
    }
    
    fn enqueue(&mut self,value : T) {
        let mut new_node = Box::new(Node{
            elem:value,
            next:None,
                });
        
        let raw_node:*mut Node<T> = &mut *new_node; 
        if self.head.is_none(){
            self.head = Some(new_node);
        } else {
            let tail = self.tail.unwrap();
            unsafe{
                (*tail).next = Some(new_node);
            }
        }
        self.tail = Some(raw_node);
        self.size+=1;
        
    }
    
    
    fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|mut node|{
            self.head = node.next.take();
            if self.head.is_none() {
                self.tail = None;
            }
            self.size -= 1;
            node.elem
        })
        
    }
    fn len(&self) -> usize {
        self.size
    } 
    
    
    fn peek(&self) ->Option<&T> {
        
        self.head.as_ref().map(|node|&node.elem)
    }
}

fn main(){
    
    let mut queue = Queue::<i32>::new();
    queue.enqueue(12);
    queue.enqueue(2);
    queue.enqueue(32);
    queue.enqueue(19);
    queue.enqueue(90);
    println!("Length : {}",queue.len());
    println!("Top : {:?}",queue.peek());
    println!("Dequeue : {:?}",queue.dequeue());
    println!("Dequeue : {:?}",queue.dequeue());
    println!("Dequeue : {:?}",queue.dequeue());
    println!("Length : {}",queue.len());
    println!("Top : {:?}",queue.peek());
    
}