fn main() {

    let some_value = Some(48); 
    let value = some_value.unwrap();
    println!("value {}",value);
    
     let non_value = None;
    // let value = non_value.unwrap();
    // println!("value {}",value);
    let value1 = non_value.unwrap_or(4);
    let value2 = non_value.unwrap_or_else(|| 3);
    
    println!("Value 1 : {} , Value 2 : {}",value1,value2);
    let value3 = some_value.map(|x| x/2);
    println!("Mapped Value : {}",value3.unwrap_or(10));
    
    let new_value = some_value.and_then(|x| if x > 50 { Some(50)} else { Some(100)});
    println!("Option And Then  Value : {}",new_value.unwrap_or(10));
    println!("Value Check : {} ",some_value.is_some());
    println!("Value Check : {} ",some_value.is_none());
    println!("Value Check : {} ",non_value.is_some());
    println!("Value Check : {} ",non_value.is_none());
    
    let filter1 = some_value.filter(|&x| x>40);
    let filter2 = some_value.filter(|&x| x<40);
    println!("Filter  Value : {}",filter1.unwrap_or(0));
    println!("Filter Value : {}",filter2.unwrap_or(0));
    
    println!("Or  Value : {}",some_value.or(Some(0)).unwrap());
    println!("Or  Value : {}",non_value.or(Some(200)).unwrap());
    println!("Or Else  Value : {}",non_value.or_else(||Some(1000)).unwrap());
    
    let result1 = some_value.ok_or("Error!");
    let result2 = non_value.ok_or("Error2!");
    println!("Result 1");
    match result1 {
      Ok(value) => println!("Value receieved  {} ",value),
      Err(msg) => println!("Error receieved  {} ",msg),
    }
    
    println!("Result2");
    match result2 {
      Ok(value) => println!("Value receieved  {} ",value),
      Err(msg) => println!("Error receieved  {} ",msg),
    }
    
    
    
    
}

