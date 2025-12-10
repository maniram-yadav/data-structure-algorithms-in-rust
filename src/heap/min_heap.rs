use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let mut min_heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();

    // Push elements into the heap
    min_heap.push(Reverse((5, 100)));
    min_heap.push(Reverse((2, 200)));
    min_heap.push(Reverse((2, 1))); // Tie-breaker for '2'
    min_heap.push(Reverse((10, 50)));

    // Pop elements: they come out in *ascending* order of the i32 value
    // (and ascending usize for ties)
    assert_eq!(min_heap.pop(), Some(Reverse((2, 1))));
    assert_eq!(min_heap.pop(), Some(Reverse((2, 200))));
    assert_eq!(min_heap.pop(), Some(Reverse((5, 100))));
    assert_eq!(min_heap.pop(), Some(Reverse((10, 50))));
    assert_eq!(min_heap.pop(), None);
}
