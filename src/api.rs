use tokio::sync::mpsc::UnboundedSender;

use crate::{
    data::{Todo, User},
    update::Message,
};

const USERS_URL: &str = "https://jsonplaceholder.typicode.com/users";

pub fn spawn_fetch_users(tx: UnboundedSender<Message>) {
    tokio::spawn(async move {
        let message = match fetch_users().await {
            Ok(users) => Message::UsersLoaded(users),
            Err(errore) => Message::UsersLoadFailed(errore.to_string()),
        };

        let _ = tx.send(message);
    });
}

async fn fetch_users() -> Result<Vec<User>, reqwest::Error> {
    reqwest::get(USERS_URL).await?.json().await
}

pub fn spawn_fetch_todos(tx: UnboundedSender<Message>, user_id: u32) {
    tokio::spawn(async move {
        let message = match fetch_todos(user_id).await {
            Ok(todos) => Message::TodosLoaded(todos),
            Err(errore) => Message::TodosLoadFailed(errore.to_string()),
        };

        let _ = tx.send(message);
    });
}

async fn fetch_todos(user_id: u32) -> Result<Vec<Todo>, reqwest::Error> {
    let url = format!("https://jsonplaceholder.typicode.com/todos?userId={user_id}");
    reqwest::get(url).await?.json().await
}
