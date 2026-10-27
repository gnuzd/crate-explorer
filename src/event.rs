use std::{sync::mpsc, thread, time::Duration};

use color_eyre::Result;
use ratatui::crossterm::event::{self, Event as CtEvent, KeyEvent, KeyEventKind};

#[derive(Debug)]
pub enum Event {
    Key(KeyEvent),
    Tick,
}

pub struct Events {
    rx: mpsc::Receiver<Event>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            loop {
                let sent = if event::poll(tick_rate).unwrap_or(false) {
                    match event::read() {
                        Ok(CtEvent::Key(k)) if k.kind == KeyEventKind::Press => {
                            tx.send(Event::Key(k))
                        }
                        Ok(_) => tx.send(Event::Tick),
                        Err(_) => return,
                    }
                } else {
                    tx.send(Event::Tick)
                };

                if sent.is_err() {
                    return;
                }
            }
        });

        Self { rx }
    }

    pub fn next(&self) -> Result<Event> {
        Ok(self.rx.recv()?)
    }
}
