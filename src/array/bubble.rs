
pub fn sort(arr :&mut Vec<i8>){
    println!("Using Bubble sorting for arr {:?}",arr);
    for i in 0..arr.len(){
        for j in 0..arr.len()-i-1{
            if arr[j]>arr[j+1] {
                let temp = arr[j];
                arr[j]=arr[j+1];
                arr[j+1] = temp;
            }
        }
    }
}