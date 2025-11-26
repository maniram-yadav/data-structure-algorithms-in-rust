use std::sync::Arc;
use std::thread;

// A simple wrapper around a raw file descriptor (like a C int).
// Rust makes no thread-safety assumptions about raw integers used in FFI contexts.
#[derive(Debug)]
struct FileHandle(i32);

// We must manually implement Send and Sync using `unsafe` blocks.
// This is us telling the compiler, "Trust me, I know this is safe to move and share."

// We implement `Send` so we can move ownership of the handle to a different thread.
unsafe impl Send for FileHandle {}

// We implement `Sync` so we can share references (&FileHandle) across threads safely 
// (assuming we only use thread-safe OS functions with this handle).
unsafe impl Sync for FileHandle {}

impl FileHandle {
    // In a real scenario, this would call a C function
    fn open() -> Self {
        println!("Opening file/resource...");
        FileHandle(42) // Mock file descriptor
    }
    
    fn write_data(&self, data: &str) {
        // In a real scenario, this calls a thread-safe OS write function
        println!("Writing '{}' to handle {}", data, self.0);
    }
    
    // In a real scenario, this closes the file handle
    fn close(&self) {
         println!("Closing handle {}", self.0);
    }
}

fn main() {
    let handle = FileHandle::open();
    
    // We use Arc to share ownership across threads, relying on our `Sync` implementation
    let shared_handle = Arc::new(handle);
    
    let mut handles = vec![];

    for i in 0..3 {
        let handle_clone = Arc::clone(&shared_handle);
        
        // This closure requires the inner type to be Sync so it can be accessed 
        // via a shared reference (&FileHandle) within multiple threads.
        let h = thread::spawn(move || {
            // Our manual 'Sync' impl makes this possible:
            handle_clone.write_data(&format!("data batch {}", i));
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }
    
    // Clean up the resource once all threads are done.
    shared_handle.close();
}
