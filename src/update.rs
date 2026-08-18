use crate::{
    data::{Todo, User},
    state::AppState,
};

pub enum Message {
    Quit,
    ToggleFocus,
    SelectPrevious,
    SelectNext,
    Confirm,
    Tick,
    UsersLoaded(Vec<User>),
    UsersLoadFailed(String),
    TodosLoaded(Vec<Todo>),
    TodosLoadFailed(String),
    InputError(String),
}

pub fn update(state: &mut AppState, message: Message) {
    match message {
        Message::ToggleFocus => state.toggle_focus(),
        Message::SelectPrevious => state.select_previous(),
        Message::SelectNext => state.select_next(),
        Message::Tick => state.advance_spinner(),
        Message::Confirm => state.confirm_selection(),
        Message::UsersLoaded(users) => state.set_users(users),
        Message::UsersLoadFailed(errore) => state.set_users_error(errore),
        Message::TodosLoaded(todos) => state.set_todos(todos),
        Message::TodosLoadFailed(errore) => state.set_todos_error(errore),
        _ => {}
    }
}
