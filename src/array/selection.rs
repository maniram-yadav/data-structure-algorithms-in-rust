
pub fn sort(arr :&mut Vec<i8>){
    println!("Using Selection sorting for arr {:?}",arr);
    let mut min ;
    
    for i in 0..arr.len()-1 {
        min = i;
        for j in i+1..arr.len(){
            if arr[min]>arr[j] {
                min = j

            }
        }
        if arr[min]!=arr[i]{
            let temp = arr[min];
            arr[min]=arr[i];
            arr[i] = temp;
        
        }
    }
}

