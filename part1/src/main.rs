use std::sync::{Arc, Mutex};
use std::thread;
use rand::Rng;
use std::collections::HashSet;

fn main() {
    let n = 10; 
    let visited = Arc::new(Mutex::new(HashSet::new())); 
    let counter = Arc::new(Mutex::new(0)); 
    let mut handles = vec![];
    for i in 0..n {
        let visited_clone = Arc::clone(&visited);
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut has_eaten = false;
            while {
                let count = counter_clone.lock().unwrap();
                *count < n
            } {
                let chance: i32 = rng.gen_range(0..n);
                if chance == i && !has_eaten {
                    {
                        let mut visited = visited_clone.lock().unwrap();
                        if visited.insert(i) { 
                            println!("Guest {} entered and ate the cupcake.", i);
                            has_eaten = true;

                            let mut count = counter_clone.lock().unwrap();
                            *count += 1;
                            println!("Counted a visit, total unique visits now: {}", *count);
                        }
                    }
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("All guests have visited the labyrinth and eaten the cupcake.");
}