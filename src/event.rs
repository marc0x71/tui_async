use color_eyre::Result;
use std::thread;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use tokio::sync::mpsc::UnboundedSender;

use crate::update::Message;

/// Spawns a dedicated OS thread that reads terminal events and forwards them
/// as `Message`s over the channel.
///
/// A separate thread (not a tokio task) is required here because
/// `crossterm::event::read()` is a blocking call — running it inside an
/// async task would block the whole tokio runtime.
pub fn spawn_input(tx: UnboundedSender<Message>) {
    thread::spawn(move || {
        loop {
            let message = match read_message() {
                Ok(message) => message,
                Err(errore) => Message::InputError(errore.to_string()),
            };

            let input_valido = !matches!(message, Message::InputError(_));

            if tx.send(message).is_err() || !input_valido {
                return;
            }
        }
    });
}

fn read_message() -> Result<Message> {
    loop {
        let event = event::read()?;

        let maybe_message = match event {
            Event::Key(key_event) if key_event.is_press() => key_to_message(key_event.code),

            // Force a redraw on resize by reusing `Message::Tick`.
            Event::Resize(_, _) => Some(Message::Tick),

            // Add handling for other event kinds here (e.g. `Event::Mouse`).
            _ => continue,
        };
        if let Some(message) = maybe_message {
            return Ok(message);
        }
    }
}

fn key_to_message(code: KeyCode) -> Option<Message> {
    match code {
        KeyCode::Esc | KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Enter => Some(Message::Confirm),
        KeyCode::Tab => Some(Message::ToggleFocus),
        KeyCode::Up => Some(Message::SelectPrevious),
        KeyCode::Down => Some(Message::SelectNext),
        _ => None,
    }
}
