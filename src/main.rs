extern crate inputbot;

use inputbot::handle_input_events;
use inputbot::KeybdKey::OtherKey;

use std::io::{self, Read, Write};
use std::vec::Vec;
use std::string::String;

fn write_message(msg: &str) -> () {
    let encoded_len = (msg.len() as u32).to_ne_bytes();
    let encoded_msg = msg.as_bytes();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();
    stdout_lock.write_all(&encoded_len).unwrap();
    stdout_lock.write_all(&encoded_msg).unwrap();
    stdout_lock.flush().unwrap();
}

fn read_message() -> String {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    
    let mut encoded_len: [u8; 4] = [0, 0, 0, 0];
    stdin_lock.read_exact(&mut encoded_len).unwrap();
    let len = u32::from_ne_bytes(encoded_len);
    
    let mut encoded_msg: Vec<u8> = Vec::new();
    encoded_msg.resize(len as usize, 0);
    stdin_lock.read_exact(&mut encoded_msg).unwrap();

    return String::from_utf8(encoded_msg).unwrap();
}

fn main() {
    // Here we block until at least one message is read
    // It's kinda fine because extension sends messages when opening a new tab
    // The only case when we block for a long time is when already runnning extension tries
    // to reconnect to this application, which is not very common
    // It could be a good idea to send message when starting native application in any mode
    let msg = read_message();

    // I don't think using JSON library is really needed here
    if msg.contains("ping") {
        write_message(r#""pong""#);
    }
    
    OtherKey(0xB0).bind(|| {
        eprintln!("Keypress: Next track");      
        write_message(r#"{"command": "next", "argument": null}"#);
    });
    OtherKey(0xB1).bind(|| {
        eprintln!("Keypress: Prev track");
        write_message(r#"{"command": "previous", "argument": null}"#);
    });
    OtherKey(0xB2).bind(|| {
        eprintln!("Keypress: Stop");
	write_message(r#"{"command": "stop", "argument": null}"#);
    });
    OtherKey(0xB3).bind(|| {
        eprintln!("Keypress: Play/Pause");
	write_message(r#"{"command": "playPause", "argument": null}"#);
    });
    handle_input_events();

    // We ought to be listening forever
    unreachable!();
}
