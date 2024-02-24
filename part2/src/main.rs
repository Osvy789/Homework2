use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

struct Showroom {
    is_available: bool,
    queue: Vec<usize>, 
}
impl Showroom {
    fn new() -> Showroom {
        Showroom {
            is_available: true,
            queue: Vec::new(),
        }
    }
}
fn main() {
    let n = 10; 
    let showroom = Arc::new((Mutex::new(Showroom::new()), Condvar::new()));
    let mut handles = vec![];
    for id in 0..n {
        let showroom_clone = Arc::clone(&showroom);
        let handle = thread::spawn(move || {
            let (lock, cvar) = &*showroom_clone;
            let mut showroom = lock.lock().unwrap();
            showroom.queue.push(id);
            while showroom.queue[0] != id || !showroom.is_available {
                showroom = cvar.wait(showroom).unwrap();
            }
            showroom.is_available = false;
            println!("Guest {} is viewing the vase.", id);
            thread::sleep(Duration::from_secs(1)); 
            showroom.is_available = true;
            showroom.queue.remove(0); 
            println!("Guest {} has left the showroom.", id);
            cvar.notify_all(); 
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
