use figlet_rs::{FIGfont};
use chrono::{Local, Timelike};
use std::{process::Command, thread, time};

fn main() {

    let font = FIGfont::standard().unwrap();

    while true {
        let now = Local::now();
        let time_format = now.format("%H:%M:%S").to_string();
        let output = Command::new("clear").status().expect("Failed to execute 'clear'");
        let figure = font.convert(&time_format);
        assert!(figure.is_some());
        println!("{}", figure.unwrap());
        thread::sleep(time::Duration::from_millis(100));
    }

}

