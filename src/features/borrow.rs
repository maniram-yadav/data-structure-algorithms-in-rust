use std::borrow::Borrow;

fn print_string_length<T:Borrow<str>>(t : T){
    println!("Length: {} ",t.borrow().len());
}


mod test {

    use super::*;
    #[test]
    fn test_borrow() {
          
            let owned_string = String::from("Hello, world!");
            let borrowed_str = "Rust is great!";
            // Call with an owned String
            print_string_length(owned_string);
            // Call with a borrowed &str
            print_string_length(borrowed_str);
    } 
}
