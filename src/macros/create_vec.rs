macro_rules! create_vec {
    
    ($($arg:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($arg);
            )*
            temp_vec
        }
       };
}



fn main(){
    let arr = create_vec!(1,2,6,23);
    println!("{:?}",arr);
}