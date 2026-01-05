
//--- START OF FILE: main_refcell.rs ---




use std::cell::RefCell;


fn main() {

    let my_value = RefCell::new(10);
    let borrow_immutably = my_value.borrow();

    println!("Initial value : {}",*borrow_immutably);
    drop(borrow_immutably);

    {
        let mut borrow_mutably = my_value.borrow_mut();
        *borrow_mutably += 5;
    }
    let borrow_immutably_after_mod = my_value.borrow();
    println!("Value after mutation : {}",*borrow_immutably_after_mod);
   
    drop(borrow_immutably_after_mod);
    let mut borrow_mutably = my_value.borrow_mut();
    *borrow_mutably += 5;
    drop(borrow_mutably);    
    let borrow_immutably_after_mod = my_value.borrow();
    println!("Value after mutation : {}",*borrow_immutably_after_mod);
    
}

//--- END OF FILE: main_refcell.rs ---

