use std::io;
use std::sync::mpsc;
use std::thread;

use termion::event::Key;
use termion::input::TermRead;


pub fn events() -> mpsc::Receiver<Key> {
    let (tx, rx) = mpsc::channel();
    let keys_tx = tx.clone();
    thread::spawn(move || {
        let stdin = io::stdin();
        for evt in stdin.keys() {
            if let Ok(key) = evt {
                if let Err(err) = keys_tx.send(key) {
                    eprintln!("{}", err);
                    return;
                }
            }
        }
    });
    rx
}
