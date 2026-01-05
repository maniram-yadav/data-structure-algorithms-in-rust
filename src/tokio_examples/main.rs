
//--- START OF FILE: main.rs ---

use std::fs;
use std::path;

trait FileMetadata {
    fn exists(&self) -> bool;
    fn is_readable(&self) -> bool;
    fn is_writeable(&self) -> bool;
}


impl FileMetadata for path::Path {

    fn is_readable(&self) -> bool {
        fs::File::open(self).is_ok()
    }
    
    fn is_readable(&self) -> bool {
        
        // first way
        // if let Ok(m) = fs::metadata(self) {
        //         !m.permissions.readonly()
        // } else {
        //     false
        // }

        //second way
        fs::metadata(self)
            .map(|x| {
                s.permissions.readonly()
            })
            .unwrap_or(false)

    }
    
    fn exists(&self) -> bool {
            self.exists()
    }
}


fn main(){
      
      
}


//--- END OF FILE: main.rs ---










