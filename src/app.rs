use std::time::Duration;

use color_eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

use crate::{
    action::Action,
    event::{Event, Events},
    ui::{self, theme::Theme},
};

pub struct App {
    pub theme: Theme,
    pub help_open: bool,
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            theme: Theme::crab(),
            help_open: false,
            should_quit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let events = Events::new(Duration::from_millis(250));

        while !self.should_quit {
            terminal.draw(|f| ui::draw(f, self))?;

            if let Event::Key(key) = events.next()? {
                if let Some(action) = self.map_key(key) {
                    self.update(action);
                }
            }
        }

        Ok(())
    }

    fn map_key(&self, key: KeyEvent) -> Option<Action> {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return Some(Action::Quit);
        }

        match key.code {
            KeyCode::Char('q') => Some(Action::Quit),
            KeyCode::Char('?') => Some(Action::ToggleHelp),
            KeyCode::Esc if self.help_open => Some(Action::ToggleHelp),
            _ => None,
        }
    }

    fn update(&mut self, action: Action) {
        match action {
            Action::Quit => self.should_quit = true,
            Action::ToggleHelp => self.help_open = !self.help_open,
        }
    }
}
