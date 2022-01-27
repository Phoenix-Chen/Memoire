use std::io;
use std::sync::mpsc;
use std::thread;

use termion::event::Key;
use termion::input::TermRead;


pub fn events() -> mpsc::Receiver<Key> {
    let (tx, rx) = mpsc::channel();
    let keys_tx = tx;
    thread::spawn(move || {
        let stdin = io::stdin();
        for key in stdin.keys().flatten() {
            if let Err(err) = keys_tx.send(key) {
                eprintln!("{}", err);
                return;
            }
        }
    });
    rx
}
