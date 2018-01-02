use std::sync::mpsc::sync_channel;
use std::thread;
use std::time::Duration;

struct WiggleRequest;

fn main() {

    let (sender, receiver) = sync_channel(1);

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5));
            sender.send(WiggleRequest).unwrap();
        }
    });

    loop {
        match receiver.recv() {
            Ok(WiggleRequest)   => println!("Wiggle Wiggle Wiggle!"),
            Err(_)              => println!("Something went wrong while wiggling!")
        }
    }

}
