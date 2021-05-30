use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crossterm::event::{poll, read, Event as CEvent};

pub enum Event<I> {
    Input(I),
    Tick,
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event<CEvent>>,
    _input_handle: thread::JoinHandle<()>,
    _tick_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Events {
    pub fn new() -> Events {
        Events::with_config(Config::default()).unwrap()
    }

    pub fn with_config(config: Config) -> crossterm::Result<Events> {
        let (tx, rx) = mpsc::channel();
        let _input_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                loop {
                    if poll(config.tick_rate).unwrap() {
                        if let Ok(evt) = read() {
                            if let Err(err) = tx.send(Event::Input(evt)) {
                                eprintln!("{}", err);
                                return;
                            }
                        }
                    }
                    else {}
                }
            })
        };
        let _tick_handle = {
            thread::spawn(move || loop {
                if tx.send(Event::Tick).is_err() {
                    break;
                }
                thread::sleep(config.tick_rate);
            })
        };
        Ok(Events {
            rx,
            _input_handle,
            _tick_handle,
        })
    }

    pub fn next(&self) -> Result<Event<CEvent>, mpsc::RecvError> {
        self.rx.recv()
    }
}
