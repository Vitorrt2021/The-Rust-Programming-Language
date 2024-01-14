use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;
use std::sync::mpsc;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

    let thread_2 = thread_with_move();
    thread_2.join().unwrap();


    let thread_3 = message_passing();
    thread_3.join().unwrap();
}

fn thread_with_move()-> JoinHandle<()> {
    let v = vec![1, 2, 3];

    // By adding the move keyword before the closure, we force the closure to take ownership of the values it’s using rather than allowing Rust to infer that it should borrow the values
    thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    })
}


fn message_passing()-> JoinHandle<()> {
    let (tx, rx) = mpsc::channel();

    let received = thread::spawn(move || {
        loop {
            match rx.try_recv() {
                Ok(received) => {
                    println!("Got: {}", received);
                    break;
                }
                Err(_) => {
                    println!("Error: Not received");
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }
    });
    thread::sleep(Duration::from_secs(1));

    let tx2 = tx.clone();
    thread::spawn(move || {
        let val = String::from("hi 2");
        match tx2.send(val) {
            Ok(received) => println!("send value 2"),
            Err(err) => println!("Error: {}", err)
        }
    });

    thread::spawn(move || {
        let val = String::from("hi 1");
        match tx.send(val) {
            Ok(received) => println!("send value 1"),
            Err(err) => println!("Error: {}", err)
        }
    });

    received
}