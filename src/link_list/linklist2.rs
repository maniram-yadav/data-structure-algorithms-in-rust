link list implementation ==============
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data: data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.head = old_head.next;
            self.size -= 1;
            old_head.data
        })
    }

    fn len(&self) -> usize {
        self.size
    }

    /// Adds a new element to the end of the list (O(n) operation in safe Rust).
    fn add_at_tail(&mut self, data: T) {
        let new_node = Some(Box::new(Node { data, next: None }));
        self.size += 1;

        // If the list is empty, the new node is the head.
        if self.head.is_none() {
            self.head = new_node;
            return;
        }

        // Traverse to find the last node using a mutable reference
        let mut current = self.head.as_mut();
        while let Some(node) = current {
            if node.next.is_none() {
                // We found the tail, insert the new node here
                node.next = new_node;
                return;
            }
            // Move the mutable reference forward
            current = node.next.as_mut();
        }
    }

    /// Inserts an element at a specific index (0-based).
    /// If index > list size, it appends to the tail.
    fn add_at_position(&mut self, data: T, index: usize) {
        if index == 0 {
            self.push_front(data);
            return;
        }

        if index >= self.size {
            self.add_at_tail(data);
            return;
        }

        // Traverse to the node *before* the insertion point
        let mut current = self.head.as_mut();
        for _ in 0..index - 1 {
            if let Some(node) = current {
                current = node.next.as_mut();
            } else {
                // This path should ideally not be reached due to the index check
                return; 
            }
        }

        // `current` is now the node just before the insertion index.
        if let Some(node) = current {
            // Take the current node's next value, creating a gap
            let next_node = node.next.take();
            
            // Create the new node and point its next to the rest of the list
            let new_node = Box::new(Node {
                data,
                next: next_node,
            });

            // Point the current node's next to the new node
            node.next = Some(new_node);
            self.size += 1;
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current_node = self.head.take();
        while let Some(mut boxed_node) = current_node {
            current_node = boxed_node.next.take();
        }
    }
}

fn main() {
    let mut list = LinkedList::<i32>::new();

    list.add_at_tail(10);
    list.add_at_tail(20);
    list.push_front(5); // List: [5, 10, 20]
    println!("List size after setup: {}", list.len()); // Output: 3

    // Add at specific position
    list.add_at_position(15, 2); // List: [5, 10, 15, 20]
    list.add_at_position(1, 0);  // Add at head: [1, 5, 10, 15, 20]
    list.add_at_position(30, 10); // Add at tail (index out of bounds): [1, 5, 10, 15, 20, 30]

    println!("List size after insertions: {}", list.len()); // Output: 6
    
    // Demonstrate contents by popping all elements (requires an iterator in a real use case)
    while let Some(data) = list.pop_front() {
        println!("Popped: {}", data);
    }
    }