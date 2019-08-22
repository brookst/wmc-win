extern crate inputbot;
extern crate ws;

use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;

use inputbot::handle_input_events;
use inputbot::KeybdKey::OtherKey;
use ws::{listen, util::Token, CloseCode, Error, ErrorKind, Handler, Handshake, Message, Sender};

const POLL: Token = Token(1);
const UPDATE_MS: u64 = 100;

struct RainmeterListener {
    ws: Sender,
    input: Arc<Mutex<Receiver<String>>>,
}

impl Handler for RainmeterListener {
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        println!("Recieved message: {}", msg);
        Ok(())
    }
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        println!("Connection established");
        // exhaust any kepresses prior to connection
        while let Ok(_) = self.input.lock().unwrap().try_recv() {}
        // schedule a timeout to send a gratuitous pong every 5 seconds
        self.ws.timeout(UPDATE_MS, POLL)
    }
    fn on_timeout(&mut self, event: Token) -> ws::Result<()> {
        if event == POLL {
            if let Ok(command) = self.input.lock().unwrap().try_recv() {
                println!("Sending command: {}", command);
                self.ws.send(command)?;
            }
            // reschedule the timeout
            self.ws.timeout(UPDATE_MS, POLL)
        } else {
            Err(Error::new(
                ErrorKind::Internal,
                "Invalid timeout token encountered!",
            ))
        }
    }
    fn on_close(&mut self, _: CloseCode, _: &str) {
        println!("Connection lost");
    }
}

fn main() -> ws::Result<()> {
    // Channel to pass keypresses to websocket
    let (key_in, key_out) = channel();
    let key_stream = Arc::new(Mutex::new(key_out));

    // Listen for keypresses of interest
    thread::Builder::new()
        .name("key_listener".to_owned())
        .spawn(|| {
            let output = Arc::new(Mutex::new(key_in));
            OtherKey(0xB0).bind({
                let output = output.clone();
                move || {
                    println!("Keypress: Next track");
                    let _ = output.lock().unwrap().send("next".to_owned());
                }
            });
            OtherKey(0xB1).bind({
                let output = output.clone();
                move || {
                    println!("Keypress: Prev track");
                    let _ = output.lock().unwrap().send("previous".to_owned());
                }
            });
            OtherKey(0xB2).bind({
                let output = output.clone();
                move || {
                    println!("Keypress: Stop");
                    let _ = output.lock().unwrap().send("pause".to_owned());
                }
            });
            OtherKey(0xB3).bind({
                let output = output.clone();
                move || {
                    println!("Keypress: Play/Pause");
                    let _ = output.lock().unwrap().send("playpause".to_owned());
                }
            });
            handle_input_events();
        })
        .unwrap();

    // Listen for websocket connections from web-media-controller
    listen("127.0.0.1:8974", |out| RainmeterListener {
        ws: out,
        input: key_stream.clone(),
    })?;

    // We ought to be listening forever
    unreachable!();
}
