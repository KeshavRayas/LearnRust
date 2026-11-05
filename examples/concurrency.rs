// Threads, channels, and shared state.
//     cargo run --example concurrency
//
// Rust's ownership rules prevent data races at compile time. Two traits decide
// what can cross a thread boundary:
//   Send    safe to move to another thread (most types)
//   Sync    safe to share &T between threads (most types)
// Rc<T> and RefCell<T> are neither, so the compiler stops you from using them
// across threads.

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

fn main() {
    // 1. Spawn and join. A panic in the thread is returned as an Err on join.
    let handle = thread::spawn(|| println!("hello from a thread"));
    handle.join().unwrap();

    // 2. `move` transfers captured values into the closure, which is required
    //    because the thread may outlive this function's stack frame.
    let msg = String::from("owned by the thread");
    let h = thread::spawn(move || println!("{msg}"));
    h.join().unwrap();
    // println!("{msg}"); // error[E0382]: moved

    // 3. Channels: message passing. Multiple producers, one consumer.
    let (tx, rx) = mpsc::channel();
    for id in 0..3u64 {
        let tx = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(10 * id));
            tx.send(format!("worker {id} done")).unwrap();
        });
    }
    drop(tx); // close the original sender or the for loop below never ends
    for m in rx {
        println!("{m}");
    }

    // 4. Shared mutable state: Arc (atomic reference count) + Mutex.
    //    Use this when threads must mutate the same value. Prefer channels or
    //    returning values from threads when you can.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut n = counter.lock().unwrap(); // blocks until the lock is free
            *n += 1;
        })); // guard drops here, lock released
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("counter = {}", *counter.lock().unwrap());

    // Deadlock warning: lock() twice in the same thread without dropping the
    // first guard, and you hang. std::sync::Mutex is not reentrant.

    // 5. Scoped threads (Rust 1.63+) can borrow from the stack instead of move.
    let data = vec![1, 2, 3];
    thread::scope(|s| {
        s.spawn(|| println!("scoped thread sees {data:?}"));
        s.spawn(|| println!("and so does this one: {}", data.len()));
    }); // all scoped threads are joined here
}
