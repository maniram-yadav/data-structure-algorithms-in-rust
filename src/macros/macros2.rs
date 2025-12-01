macro_rules! print_hello {
    
    () => {
        println!("Hello !");
    };
    
     ($name:expr) => {
        println!("Hello {}!",$name);
    };
    
    
    ($($arg:expr),*) => {
        $(print!("{}, ",$arg);)*
        // let count = count!($($arg),*);
       };
}


macro_rules! print_arg_count {
    ($($args:expr),*) => {
        // Create a temporary array of unit `()` for each argument and get the length.
        // This is evaluated at compile time.
         const COUNT: usize = [$(1 as usize),*].len(); 
         
        println!("Number of arguments: {}", COUNT);
    
       
    };
}


// macro_rules! args_count{
//     ($_head:tt ($tail:tt)*) => { 1usize + args_count!($($tail)*) };
// }

fn main(){
    print_hello!();
    print_hello!("Ravi");
    print_hello!("Ravi","Shyam","Smith");
    print_arg_count!("Ravi", "Shyam", "Smith");
}