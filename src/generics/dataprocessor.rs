use std::ops::Add;

trait DataProcessor<T> { 
  fn process(&self,data:&[T]) -> T;
}


struct SumProcessor;
impl<T> DataProcessor<T> for SumProcessor 
  where T : Add<Output=T> + Copy + Default  
 {  
  fn process(&self,data:&[T]) -> T{
    data.iter().copied().fold(T::default(),Add::add)
  }
} 

struct MaxProcessor;
impl<T> DataProcessor<T> for  MaxProcessor 
  where T:PartialOrd + Copy
  {
   
    fn process(&self,data :&[T]) -> T {
      data.iter().copied().fold(data[0],|max,val| if val > max { val } else { max })    
    }
  }

struct ConcatProcessor;
impl ConcatProcessor {
   fn hello(&self) -> &str {
     "hello"
   }
}


impl DataProcessor<String> for ConcatProcessor {
  
   fn process(&self, data : &[String]) -> String {
      
       data.join("")
   }
  
}


fn main() {
  
    let int_arr = vec![1,3,6,23,9];
    
    let fl_arr = vec![1.5,3.12,6.23,1.9];
    
    let str_arr = vec!["hello".to_string(),"raj".to_string()];
    
    let sumprocessor = SumProcessor;
    let maxprocessor = MaxProcessor;
    let concatprocessor = ConcatProcessor;
    
    println!("Sum of numbers {}",sumprocessor.process(&int_arr));
    println!("Sum of floats {}",sumprocessor.process(&fl_arr));
    
    
    println!("Max of numbers {}",maxprocessor.process(&int_arr));
    println!("Max of floats {}",maxprocessor.process(&fl_arr));

     println!("Concat of Strings {}",concatprocessor.process(&str_arr));
     println!("Hello test {}",concatprocessor.hello());
    
}





