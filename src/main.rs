extern crate clap;
extern crate enigo;
extern crate rand;

use clap::{Arg, App};
use enigo::*;
use rand::distributions::{IndependentSample, Range};
use std::str::FromStr;
use std::sync::mpsc::sync_channel;
use std::thread;
use std::time::Duration;

struct WiggleRequest;

fn main() {
    let matches = App::new("Wiggler")
        .version("0.1")
        .author("romatthe <robin.mattheussen@gmail.com>")
        .about("Periodically wiggles the mouse")
        .arg(Arg::with_name("time")
            .short("t")
            .long("time")
            .value_name("DURATION")
            .help("Set the interval between each mouse wiggle")
            .takes_value(true)
            .required(true)
            .validator(is_duration))
        .get_matches();

    let (sender, receiver) = sync_channel(1);
    let seconds = u64::from_str(matches.value_of("time").unwrap()).unwrap();

    println!("Wiggle wiggle wiggle! Moving the mouse every {} seconds", seconds);

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(seconds));
            sender.send(WiggleRequest).unwrap();
        }
    });

    let between = Range::new(-1000, 1000);
    let mut rng = rand::thread_rng();
    let mut enigo = Enigo::new();

    loop {
        match receiver.recv() {
            Ok(WiggleRequest) => {
                enigo.mouse_move_relative(between.ind_sample(&mut rng), between.ind_sample(&mut rng));
                enigo.mouse_move_relative(between.ind_sample(&mut rng), between.ind_sample(&mut rng));
                enigo.mouse_move_relative(between.ind_sample(&mut rng), between.ind_sample(&mut rng));
            },

            Err(_) => println!("Something went wrong while wiggling!")
        }
    }
}

fn is_duration(val: String) -> Result<(), String> {
    let result = u64::from_str(&val);

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err("Failed to parse the duration. Please provide a duration in seconds.".to_string())
    }
}
