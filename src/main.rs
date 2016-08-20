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

fn main() {
    println!("Hello world!");

    let mut port = serial::posix::TTYPort::open(&Path::new("/dev/ttymxc3")).unwrap();
    configure_serial(&mut port).unwrap();

    loop {
        match serial_read(&mut port) {
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

fn serial_read<T: SerialPort>(port: &mut T) -> io::Result<()> {  
    //let mut buf = String::new();
    //port.read_to_string(&mut buf);

    let mut buf: Vec<u8> = (0..32).collect();
    port.read(&mut buf[..]);
    let mut buf = String::from_utf8(buf).unwrap();

    let re = Regex::new(r"volume: (\d+)").unwrap();
    for cap in re.captures_iter(&buf) {
        let vol: u8 = cap.at(1).unwrap_or("").parse().unwrap();
        println!("vol: {}", vol);
    }

    Ok(())
}

/*
fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
    
    try!(port.write_all(b"\rsub serialout volume\r"));
    try!(port.flush());

    thread::sleep(Duration::from_millis(100));

    Ok(())
}
*/
