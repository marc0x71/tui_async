mod api;
mod data;
mod event;
mod state;
mod ui;
mod update;

use color_eyre::{Result, eyre::bail};
use std::time::Duration;

use ratatui::DefaultTerminal;
use tokio::sync::mpsc;

use crate::{state::AppState, update::Message};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();

    let result = run(&mut terminal).await;

    ratatui::restore();

    result
}

async fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut state = AppState::default();
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    api::spawn_fetch_users(tx.clone());
    event::spawn_input(tx.clone());

    let mut tick = tokio::time::interval(Duration::from_millis(100));

    loop {
        terminal.draw(|frame| ui::render(frame, &mut state))?;

        tokio::select! {
            Some(message) = rx.recv() => {
                match message {
                    Message::Quit => break,
                    Message::InputError(errore) => bail!(errore),
                    Message::Confirm if state.is_list_focused() => {
                        update::update(&mut state, message);

                        if let Some(user_id) = state.current_user_id() {
                            api::spawn_fetch_todos(tx.clone(), user_id);
                        }
                    }

                    Message::Confirm => {}

                    message => {
                        update::update(&mut state, message);
                    }
                }
            }

            _ = tick.tick() => {
                update::update(&mut state, Message::Tick);
            }
        }
    }

    Ok(())
}
