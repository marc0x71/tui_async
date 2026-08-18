use std::time::Duration;

use color_eyre::eyre::Result;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    data::{Todo, User},
    update::Message,
};

const USERS_URL: &str = "https://jsonplaceholder.typicode.com/users";
pub struct ApiClient {
    client: reqwest::Client,
    tx: UnboundedSender<Message>,
}

impl ApiClient {
    pub fn new(tx: UnboundedSender<Message>) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;

        Ok(Self { client, tx })
    }

    pub fn spawn_fetch_users(&self) {
        let client = self.client.clone();
        let tx = self.tx.clone();
        tokio::spawn(async move {
            let message = match fetch_users(&client).await {
                Ok(users) => Message::UsersLoaded(users),
                Err(errore) => Message::UsersLoadFailed(errore.to_string()),
            };

            let _ = tx.send(message);
        });
    }

    pub fn spawn_fetch_todos(&self, user_id: u32) {
        let client = self.client.clone();
        let tx = self.tx.clone();
        tokio::spawn(async move {
            let message = match fetch_todos(&client, user_id).await {
                Ok(todos) => Message::TodosLoaded(todos),
                Err(errore) => Message::TodosLoadFailed(errore.to_string()),
            };

            let _ = tx.send(message);
        });
    }
}

async fn fetch_users(client: &reqwest::Client) -> Result<Vec<User>, reqwest::Error> {
    client.get(USERS_URL).send().await?.json().await
}

async fn fetch_todos(client: &reqwest::Client, user_id: u32) -> Result<Vec<Todo>, reqwest::Error> {
    let url = format!("https://jsonplaceholder.typicode.com/todos?userId={user_id}");
    client.get(url).send().await?.json().await
}
