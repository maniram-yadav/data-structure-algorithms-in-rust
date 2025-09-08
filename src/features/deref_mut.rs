use std::ops::{Deref,DerefMut};

struct MyMutableBox<T>{
    value : T,
}

impl<T> Deref for MyMutableBox<T> {

    type Target = T;

    fn deref(&self)->  &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for MyMutableBox<T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}


mod test {

    use super::*;

    #[test]
    fn test_deref_mut(){
        let mut my_box = MyMutableBox{value:10};
     
        println!("Value {}",my_box.value);
        *my_box = 20; 
        println!("Modified Value {}",my_box.value);
        let original = *my_box;
        println!("Original Value {}",original)
        
    }
}