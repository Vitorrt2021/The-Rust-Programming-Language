use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;


fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    multiple_threads_mutex();
}


fn multiple_threads_mutex(){
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let time = rand::thread_rng().gen_range(1..=100);
            thread::sleep(Duration::from_millis(time));

            let mut num = counter.lock().unwrap();

            *num += 1;

            println!("counter = {:?}  iteration = {:?}", *num, i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}