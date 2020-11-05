use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
fn main() {
    thread_example();
    message_passing();
    with_ref_count();
}

fn thread_example(){
    let t1 = thread::spawn(||{
        for i in 1..10{
            println!("Greeting {} from other thread!", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..5{
        println!("Greeting {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    t1.join().unwrap();

}
fn message_passing(){
    let (tx1, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx1);

    thread::spawn(move || {
        println!("Sending message from thread 1");
        tx1.send(String::from("greeting from thread 1")).unwrap();
    });

    thread::spawn(move||{
        println!("Sending message from thread 2");
        tx2.send(String::from("greeting from thread 2")).unwrap();
    });

    for recvd in rx{
        println!("Received: {}", recvd);
    }
}
fn with_ref_count(){
    let counter = Arc::new(Mutex::new(0));
    let mut threads = vec![];

    for _ in 0..5{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move ||{
            let mut num_guard = (*counter).lock().unwrap();
            *num_guard +=1;
        });
        threads.push(handle);
    }
    for thrd in threads{
        thrd.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}