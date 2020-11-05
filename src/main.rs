use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    thread_example();
    message_passing();
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