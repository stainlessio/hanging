use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum Event {
    Done
}

fn main() {
    let (sender, receiver) = channel();

    thread::spawn(move|| {
        let wait_and_send = || {
            println!("Sleeping in thread");
            thread::sleep(Duration::from_secs(10));
            println!("Finished, sending Done");
            Event::Done
        };
        sender.send(wait_and_send()).unwrap();
    });

    println!("Spawned Thread");
    println!("{:?}", receiver.recv().unwrap());
}
