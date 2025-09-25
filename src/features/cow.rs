use std::borrow::Cow;


fn modulo_3(input : u8 ) -> Cow<'static,str> {
  match input%3 {
    0 => "Modulo is 0".into(),
    1 => "Modulo is 1".into(),
    remainder => format!("Modulo is > {} ",remainder).into(),
  }
  
}


fn main() {
  
  for n  in  1..7 {
    match modulo_3(n) {
      Cow::Borrowed(value) => println!("Borrowed value is {} ",value),
      Cow::Owned(value) => println!("Owned value is {} ",value),
      
    }
  }
}
