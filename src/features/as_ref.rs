
struct MyStruct {
    data : String,
}


impl AsRef<str> for MyStruct {

    fn as_ref(&self) -> &str {
        &self.data
    }
}

fn print_as_str<T: AsRef<str>>(value: T) {
    let s_ref = value.as_ref();
    println!("Value as &str: {}", s_ref);
}


mod test {

    use super::*;
    #[test]
    fn test_as_ref(){
         let owned_string = String::from("Hello, world!");
    let string_slice = "Rust is great!";
    let my_struct_instance = MyStruct {
        data: String::from("Custom data here."),
        };

      print_as_str(owned_string);  
      print_as_str(string_slice);
      print_as_str(my_struct_instance);
    }
}