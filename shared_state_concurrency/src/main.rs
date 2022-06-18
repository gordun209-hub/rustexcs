// in a way, channels in any programming language are simillar
// to single ownership, because once you transfer a value down a channel
// you should no longer use that value. shared memory concurrency is
// like multiple ownership: multiple threads can access the same memory
// location at the same time.
// Rules of mutex:
// you must attempt to acquire the lock before using the data.
// when youre done with the data that the mutex guards, you must
// unlock the data so other threads can acquire the lock.
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

