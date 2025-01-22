
mod array;

fn main(){
    let  arr = vec!{91,12,3,78};

    let mut clone = arr.clone();
    array::bubble::sort(&mut clone);
    println!("Sorted array {:?}",clone);
    
    
    clone = arr.clone();
    array::selection::sort(&mut clone);
    println!("Sorted array {:?}",clone);
}