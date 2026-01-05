

//--- START OF FILE: main_cell.rs ---


use std::cell::Cell;

struct Configuration {
    version :   String,
    debug_mode : Cell<bool>,
}

fn main() {

    let app_config = Configuration{
        version:String::from("1.0.1.0"),
        debug_mode : Cell::new(false)
    };

    println!("Debug mode initially : {}",app_config.debug_mode.get());
    app_config.debug_mode.set(true);
    println!("Debug mode after change : {}",app_config.debug_mode.get());
    assert_eq!(app_config.debug_mode.get(), true);
    
}

//--- END OF FILE: main_cell.rs ---

