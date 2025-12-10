// my list reversal

fn main() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    my_vec.reverse();
    println!("{:?}", my_vec); // Output: [5, 4, 3, 2, 1]
}


fn reverse_in_place<T>(arr: &mut [T]) {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        arr.swap(left, right);
        left += 1;
        right -= 1;
    }
}

fn main() {
    let mut my_array = [1, 2, 3, 4, 5];
    my_array.as_mut_slice().reverse();
    println!("{:?}", my_array); // Output: [5, 4, 3, 2, 1]
}


fn main() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    reverse_in_place(&mut my_vec);
    println!("{:?}", my_vec); // Output: [5, 4, 3, 2, 1]

    let mut my_array = [1, 2, 3, 4, 5];
    reverse_in_place(&mut my_array);
    println!("{:?}", my_array); // Output: [5, 4, 3, 2, 1]
}

