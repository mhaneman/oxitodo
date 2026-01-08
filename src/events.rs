use crate::app::App;
use crossterm::event::{self, Event, KeyEvent};
use std::time::Duration;

pub struct EventHandler {
    poll_timeout: Duration,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            poll_timeout: Duration::from_millis(100),
        }
    }

    pub fn handle_events(&self, app: &mut App) -> color_eyre::Result<()> {
        if event::poll(self.poll_timeout)? {
            match event::read()? {
                Event::Key(key_event) => self.handle_key_event(app, key_event),
                Event::Mouse(_) => {
                    // Mouse events are currently not handled
                }
                Event::Resize(_, _) => {
                    // Terminal resize events could be handled here if needed
                }
                Event::FocusGained | Event::FocusLost => {
                    // Focus events could be handled here if needed
                }
                Event::Paste(_) => {
                    // Paste events could be handled here if needed
                }
            }
        }
        Ok(())
    }

    fn handle_key_event(&self, app: &mut App, key_event: KeyEvent) {
        app.handle_key_event(key_event);
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}
