use std::time::Duration;

use color_eyre::eyre::Result;
use log::info;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    data::{Todo, User},
    update::Message,
};

const USERS_URL: &str = "https://jsonplaceholder.typicode.com/users";

/// Example of a background task pattern: fetches data over HTTP and reports
/// results back to the main loop via the same `Message` channel used by
/// input and tick events. Replace with whatever async work your app needs
/// (a different API, a filesystem watch, a websocket, ...) — the pattern
/// stays the same.
pub struct ApiClient {
    client: reqwest::Client,
    tx: UnboundedSender<Message>,
}

impl ApiClient {
    /// Builds a new client with a default request timeout.
    pub fn new(tx: UnboundedSender<Message>) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;

        Ok(Self { client, tx })
    }

    // Example: spawns a background fetch and reports the result as a Message.
    pub fn spawn_fetch_users(&self) {
        let client = self.client.clone();
        let tx = self.tx.clone();
        tokio::spawn(async move {
            let message = match fetch_users(&client).await {
                Ok(users) => {
                    info!("Found {} users", users.len());
                    Message::UsersLoaded(users)
                }
                Err(errore) => Message::UsersLoadFailed(errore.to_string()),
            };

            let _ = tx.send(message);
        });
    }

    // Example: spawns a background fetch and reports the result as a Message.
    pub fn spawn_fetch_todos(&self, user_id: u32) {
        let client = self.client.clone();
        let tx = self.tx.clone();
        tokio::spawn(async move {
            let message = match fetch_todos(&client, user_id).await {
                Ok(todos) => {
                    info!("Found {} todos", todos.len());
                    Message::TodosLoaded(todos)
                }
                Err(errore) => Message::TodosLoadFailed(errore.to_string()),
            };

            let _ = tx.send(message);
        });
    }
}

async fn fetch_users(client: &reqwest::Client) -> Result<Vec<User>, reqwest::Error> {
    info!("Fetching users from {USERS_URL} ...");
    client.get(USERS_URL).send().await?.json().await
}

async fn fetch_todos(client: &reqwest::Client, user_id: u32) -> Result<Vec<Todo>, reqwest::Error> {
    let url = format!("https://jsonplaceholder.typicode.com/todos?userId={user_id}");
    info!("Fetching todos from {url} ...");
    client.get(url).send().await?.json().await
}
