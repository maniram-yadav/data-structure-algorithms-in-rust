use std::borrow::BorrowMut;

fn modify_first_element<T: BorrowMut<[i32]>>(mut container: T, new_value: i32) {
    let borrowed_slice = container.borrow_mut();
    if let Some(first_element) = borrowed_slice.first_mut() {
        *first_element = new_value;
    }
}

mod test {

    use super::*;
    #[test]
    fn test_borrow_mut() {
        let mut my_vec = vec![1, 2, 3];
        println!("Original Vec {:?}", my_vec);
        modify_first_element(my_vec.clone(), 7);
        println!("Modified Vec {:?}", my_vec);

        let mut another = vec![11, 12, 13];
        println!("-Original Vec {:?}", another);
        modify_first_element(&mut another[..], 7);
        println!("-Modified Vec {:?}", another);

        // Example with a mutable array
        let mut my_array: [i32; 3] = [4, 5, 6];
        println!("Original Array: {:?}", my_array);
        modify_first_element(&mut my_array[..], 100);
        println!("Modified Array: {:?}", my_array);
    }
}
