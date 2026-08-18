use std::thread;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use tokio::sync::mpsc::UnboundedSender;

use crate::update::Message;

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

fn read_message() -> std::io::Result<Message> {
    loop {
        let event = event::read()?;

        if let Event::Key(key) = event
            && key.kind == KeyEventKind::Press
            && let Some(message) = to_message(key.code)
        {
            return Ok(message);
        }
    }
}

fn to_message(code: KeyCode) -> Option<Message> {
    match code {
        KeyCode::Esc | KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Enter => Some(Message::Confirm),
        KeyCode::Tab => Some(Message::ToggleFocus),
        KeyCode::Up => Some(Message::SelectPrevious),
        KeyCode::Down => Some(Message::SelectNext),
        _ => None,
    }
}
