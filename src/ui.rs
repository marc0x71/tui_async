use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, List, Row, Table};

use ratatui::widgets::Paragraph;
use tui_logger::TuiLoggerWidget;

use crate::state::AppState;

/// Renders the whole UI into the given frame.
///
/// Called once per frame from the main loop. Layout and widgets below are
/// example content — replace with your own UI.
pub fn render(frame: &mut Frame, state: &mut AppState) {
    let constraints = if state.show_log() {
        vec![Constraint::Min(0), Constraint::Length(8)]
    } else {
        vec![Constraint::Min(0)]
    };

    let chunks = Layout::vertical(constraints).split(frame.area());

    render_ui(frame, chunks[0], state);

    if state.show_log() {
        render_log(frame, chunks[1]);
    }
}

// Log panel toggled with 'l', shown as a fixed-height bottom split.
fn render_log(frame: &mut Frame, area: Rect) {
    let log_widget = TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_info(Style::default().fg(Color::Green))
        .style_debug(Style::default().fg(Color::Gray))
        .block(Block::bordered().title(" Log "));
    frame.render_widget(log_widget, area);
}

fn render_ui(frame: &mut Frame, area: Rect, state: &mut AppState) {
    let [area_lista, area_tabella] =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(area);

    render_list(frame, area_lista, state);
    render_table(frame, area_tabella, state);
}

// Example: renders the user list on the left/top panel.
fn render_list(frame: &mut Frame, area: Rect, state: &mut AppState) {
    let blocco = Block::bordered()
        .title(" Utenti ")
        .border_style(if state.is_list_focused() {
            Style::new().yellow()
        } else {
            Style::new()
        });

    if state.is_users_loading() {
        let testo = format!("{} Caricamento utenti...", state.spinner_symbol());
        frame.render_widget(Paragraph::new(testo).block(blocco), area);
        return;
    }

    if let Some(errore) = state.users_error() {
        let testo = format!("Errore nel caricamento: {errore}");
        frame.render_widget(Paragraph::new(testo).block(blocco), area);
        return;
    }

    let blocco =
        blocco.title_bottom(Line::from(" ↑↓ seleziona · Invio mostra i to-do ").right_aligned());

    let items: Vec<String> = state
        .users()
        .unwrap_or(&[])
        .iter()
        .map(|user| user.name.clone())
        .collect();

    let lista = List::new(items)
        .block(blocco)
        .highlight_style(Style::new().reversed())
        .highlight_symbol("> ");

    frame.render_stateful_widget(lista, area, state.list_state_mut());
}

// Example: renders the to-do table for the selected user.
fn render_table(frame: &mut Frame, area: Rect, state: &mut AppState) {
    let titolo = match state.current_user_name() {
        Some(nome) => format!(" To-do di {nome} "),
        None => " To-do ".to_string(),
    };

    let blocco = Block::bordered()
        .title(titolo)
        .border_style(if state.is_table_focused() {
            Style::new().yellow()
        } else {
            Style::new()
        });

    if state.is_todos_idle() {
        let paragraph = Paragraph::new("Seleziona un utente e premi Invio.").block(blocco);
        frame.render_widget(paragraph, area);
        return;
    }

    if state.is_todos_loading() {
        let testo = format!("{} Caricamento to-do...", state.spinner_symbol());
        frame.render_widget(Paragraph::new(testo).block(blocco), area);
        return;
    }

    if let Some(errore) = state.todos_error() {
        let testo = format!("Errore nel caricamento: {errore}");
        frame.render_widget(Paragraph::new(testo).block(blocco), area);
        return;
    }

    let righe: Vec<Row> = state
        .todos()
        .unwrap_or(&[])
        .iter()
        .map(|todo| {
            let stato = if todo.completed {
                "Completato"
            } else {
                "Da fare"
            };
            Row::new([todo.id.to_string(), todo.title.clone(), stato.to_string()])
        })
        .collect();

    let larghezze = [
        Constraint::Length(5),
        Constraint::Fill(1),
        Constraint::Length(11),
    ];

    let tabella = Table::new(righe, larghezze)
        .header(Row::new(["ID", "Titolo", "Stato"]).style(Style::new().bold()))
        .block(blocco)
        .row_highlight_style(Style::new().reversed());

    frame.render_stateful_widget(tabella, area, state.table_state_mut());
}
