use std::fmt::Debug;

fn modify_first_element<T: AsMut<[i32]>>(mut container: T, new_value: i32) {
    let borrowed_slice = container.as_mut();
    if let Some(first_element) = borrowed_slice.first_mut() {
        *first_element = new_value;
    }
}

mod test {

    use super::*;
    #[test]
    fn test_borrow_mut(){
            
            let mut my_vec = vec![1,2,3];
            println!("Original Vec {:?}",my_vec);
            modify_first_element(my_vec.clone(),7);
            println!("Modified Vec {:?}",my_vec);

            let mut another:Vec<i32> = vec![11,12,13];
            println!("Original Vec {:?}",another);
            modify_first_element(&mut another,7);
            println!("Modified Vec {:?}",another);



    }
}