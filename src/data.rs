#[derive(Debug, Clone, serde::Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}
