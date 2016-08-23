// vim: set et sw=4 ts=4:

extern crate serial;
extern crate log;
extern crate regex;

use std::io;
use std::thread;
use std::time::Duration;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

use serial::prelude::*;

use regex::Regex;

const KEY_VOLUME: &'static str = "volume";


static mut VOL_PREV: u8 = 0;


struct VolumePublisher {
    volume: u8,
}


/**
 * Returns the regex for given key
 * (single numerical value)
 */
fn regex_for_key(key: &str) -> Regex {
  let mut re_str: String = format!("{}: {}", key, r"(\d+)");
  Regex::new(&re_str).unwrap()
}

fn main() {
    println!("Hello world!");

    let mut port = serial::open(&Path::new("/dev/ttymxc3")).unwrap();
    configure_serial(&mut port).unwrap();

    loop {
        listen(&mut port)
    }
}

fn configure_serial<T: SerialPort>(port: &mut T) -> io::Result<()> {
    try!(port.reconfigure(&|settings| {
        try!(settings.set_baud_rate(serial::Baud115200));
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_flow_control(serial::FlowNone);
        settings.set_stop_bits(serial::Stop1);
        Ok(())
    }));

    try!(port.set_timeout(Duration::from_millis(500)));

    Ok(())
}

fn listen_volume(stream: &String) {
    let re = regex_for_key(KEY_VOLUME);
    for cap in re.captures_iter(&stream) {
        let vol: u8 = cap.at(1).unwrap_or("").parse().unwrap();
        
        unsafe {
            if vol != VOL_PREV {
                Command::new("amixer")
                    .arg("set")
                    .arg("Speaker")
                    .arg(format!("{}%", vol))
                    .stdout(Stdio::null())
                    .spawn();
                println!("setting volume: {}", vol);

                VOL_PREV = vol;
            }
        }

    }
}


fn listen<T: SerialPort>(port: &mut T) {
    let mut stream: Vec<u8> = (0..32).collect();
    port.read(&mut stream[..]);
    let stream = String::from_utf8(stream).unwrap();

  // TODO: only listen when there are volume subscribers
  listen_volume(&stream);
}

/*
fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
    
    try!(port.write_all(b"\rsub serialout volume\r"));
    try!(port.flush());

    thread::sleep(Duration::from_millis(100));

    Ok(())
}
*/
