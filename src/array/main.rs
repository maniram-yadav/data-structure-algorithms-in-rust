mod bubble;
mod selection;
fn main(){
    let  arr = vec!{91,12,3,78};
    let mut clone = arr.clone();
    bubble::sort(&mut clone);
    println!("Sorted array {:?}",clone);
    selection::sort(&mut clone);
    println!("Sorted array {:?}",clone);
}