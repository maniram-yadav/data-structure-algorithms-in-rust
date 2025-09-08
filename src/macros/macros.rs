macro_rules! greet {

    () => {
        println!("Hello Rust!");
    };
    ($name:expr) => {
        println!("Hello {}!",$name);
    };
}

macro_rules! find_min {

    ($x:expr) => ($x);
    ($x:expr,$($y:expr),+) => {
        std::cmp::min($x,find_min!($($y),+))
    }
}

macro_rules! add {

    ($a:expr) => ($a);
    ($a:expr , $b:expr ) => {
        $a+$b
    };
    ($a:expr,$($b:tt)*) => {
        $a + add!($($b)*)
    }
}

mod test {

    use super::*;
    #[test]
    fn test_macro(){
        greet!();
        greet!("Alice");
        println!("Min value : {}",find_min!(23,89));
         println!("Total sum : {}", add!(12,12,34 ));
    }
}