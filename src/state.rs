use ratatui::widgets::{ListState, TableState};

use crate::data::{Todo, User};

#[derive(PartialEq)]
pub enum Focus {
    List,
    Table,
}

enum UsersStatus {
    Loading,
    Loaded(Vec<User>),
    Failed(String),
}

enum TodosStatus {
    Idle,
    Loading,
    Loaded(Vec<Todo>),
    Failed(String),
}

const SPINNER_FRAMES: [char; 4] = ['|', '/', '-', '\\'];

/// Application state, updated in response to `Message`s and read by the UI.
pub struct AppState {
    focus: Focus,
    list_state: ListState,
    table_state: TableState,
    users: UsersStatus,
    todos: TodosStatus,
    current_user_id: Option<u32>,
    current_user_name: Option<String>,
    spinner_frame: usize,
    show_log: bool,
}

impl AppState {
    /// Switches focus between the list and the table.
    pub fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            Focus::List => Focus::Table,
            Focus::Table => Focus::List,
        };
    }

    /// Moves the selection up in the currently focused widget.
    pub fn select_previous(&mut self) {
        match self.focus {
            Focus::List => self.list_state.select_previous(),
            Focus::Table => self.table_state.select_previous(),
        }
    }

    /// Moves the selection down in the currently focused widget.
    pub fn select_next(&mut self) {
        match self.focus {
            Focus::List => self.list_state.select_next(),
            Focus::Table => self.table_state.select_next(),
        }
    }

    pub fn is_list_focused(&self) -> bool {
        self.focus == Focus::List
    }

    pub fn is_table_focused(&self) -> bool {
        self.focus == Focus::Table
    }

    pub fn list_state_mut(&mut self) -> &mut ListState {
        &mut self.list_state
    }

    pub fn table_state_mut(&mut self) -> &mut TableState {
        &mut self.table_state
    }

    pub fn set_users(&mut self, users: Vec<User>) {
        let selezione = if users.is_empty() { None } else { Some(0) };
        self.list_state = ListState::default().with_selected(selezione);
        self.users = UsersStatus::Loaded(users);
    }

    pub fn set_users_error(&mut self, errore: String) {
        self.users = UsersStatus::Failed(errore);
    }

    pub fn confirm_selection(&mut self) {
        let indice = self.list_state.selected();

        let utente = match (self.users(), indice) {
            (Some(users), Some(indice)) => users.get(indice).map(|u| (u.id, u.name.clone())),
            _ => None,
        };

        if let Some((id, nome)) = utente {
            self.current_user_id = Some(id);
            self.current_user_name = Some(nome);
            self.todos = TodosStatus::Loading;
            self.focus = Focus::Table;
        }
    }

    pub fn set_todos(&mut self, todos: Vec<Todo>) {
        let selezione = if todos.is_empty() { None } else { Some(0) };
        self.table_state = TableState::default().with_selected(selezione);
        self.todos = TodosStatus::Loaded(todos);
    }

    pub fn set_todos_error(&mut self, errore: String) {
        self.todos = TodosStatus::Failed(errore);
    }

    /// Advances the loading spinner by one frame.
    pub fn advance_spinner(&mut self) {
        self.spinner_frame = self.spinner_frame.wrapping_add(1) % SPINNER_FRAMES.len();
    }

    pub fn is_users_loading(&self) -> bool {
        matches!(self.users, UsersStatus::Loading)
    }

    pub fn users_error(&self) -> Option<&str> {
        match &self.users {
            UsersStatus::Failed(errore) => Some(errore.as_str()),
            _ => None,
        }
    }

    pub fn users(&self) -> Option<&[User]> {
        match &self.users {
            UsersStatus::Loaded(users) => Some(users.as_slice()),
            _ => None,
        }
    }

    pub fn is_todos_idle(&self) -> bool {
        matches!(self.todos, TodosStatus::Idle)
    }

    pub fn is_todos_loading(&self) -> bool {
        matches!(self.todos, TodosStatus::Loading)
    }

    pub fn todos_error(&self) -> Option<&str> {
        match &self.todos {
            TodosStatus::Failed(errore) => Some(errore.as_str()),
            _ => None,
        }
    }

    pub fn todos(&self) -> Option<&[Todo]> {
        match &self.todos {
            TodosStatus::Loaded(todos) => Some(todos.as_slice()),
            _ => None,
        }
    }

    pub fn current_user_id(&self) -> Option<u32> {
        self.current_user_id
    }

    pub fn current_user_name(&self) -> Option<&str> {
        self.current_user_name.as_deref()
    }

    /// Returns the current spinner character.
    pub fn spinner_symbol(&self) -> char {
        SPINNER_FRAMES[self.spinner_frame]
    }

    pub fn show_log(&self) -> bool {
        self.show_log
    }

    pub fn toggle_log(&mut self) {
        self.show_log = !self.show_log;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            focus: Focus::List,
            list_state: ListState::default(),
            table_state: TableState::default(),
            users: UsersStatus::Loading,
            todos: TodosStatus::Idle,
            current_user_id: None,
            current_user_name: None,
            spinner_frame: 0,
            show_log: false,
        }
    }
}
