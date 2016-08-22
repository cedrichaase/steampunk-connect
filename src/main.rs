// vim: set et sw=4 ts=4:

extern crate serial;
extern crate log;
extern crate regex;

use std::io;
use std::thread;
use std::time::Duration;
use std::path::Path;

use serial::prelude::*;

use regex::Regex;

const KEY_VOLUME: String = String::from("volume");

/**
 * Returns the regex for given key
 * (single numerical value)
 */
fn regex_for_key(key: String) -> Regex {
  let mut re_str: String = format!("{}: {}", key, r"(\d+)");
  let re = Regex::new(&re_str).unwrap();
}

fn main() {
    println!("Hello world!");

    let mut port = serial::open(&Path::new("/dev/ttymxc3")).unwrap();
    configure_serial(&mut port).unwrap();

    loop {
        match listen(&mut port) {
            Err(_) => continue,
            result => result.unwrap(),
        };
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

    try!(port.set_timeout(Duration::from_millis(100)));

    Ok(())
}

fn listen_volume(stream: &String) {
    let re = regex_for_key(KEY_VOLUME);
    for cap in re.captures_iter(&stream) {
        let vol: u8 = cap.at(1).unwrap_or("").parse().unwrap();

        // TODO: notify subscribers
        println!("vol: {}", vol);
    }
}

fn serial_get<T: SerialPort>(port: &T) -> &String {
    let mut buf: Vec<u8> = (0..32).collect();
    port.read(&mut buf[..]);
    &String::from_utf8(buf).unwrap();
}

fn listen<T: SerialPort>(port: &mut T) -> io::Result<()> {
  let stream = serial_get(&port);

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
