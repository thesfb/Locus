// src/ui.rs
use crate::app::{App, AppMode, AppSection};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    draw_status_bar(f, app, chunks[0]);
    draw_main_content(f, app, chunks[1]);
    draw_command_line(f, app, chunks[2]);
}

fn draw_status_bar<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let mode_text = match app.mode {
        AppMode::Normal => "NORMAL",
        AppMode::Editing => "EDITING",
        AppMode::Command => "COMMAND",
        AppMode::MainMenu => "MENU",
        AppMode::Help => "HELP",
        AppMode::Renaming => "RENAME",
    };
    let mode_widget = Paragraph::new(mode_text).style(Style::default().fg(Color::Yellow));
    let mode_area = Rect::new(
        area.width.saturating_sub(mode_text.len() as u16 + 4),
        area.y + 1,
        mode_text.len() as u16 + 2,
        1,
    );
    f.render_widget(mode_widget, mode_area);

    let status_message = app
        .status_message
        .as_deref()
        .unwrap_or("Terminal Notes - Press : for commands, Ctrl+Q to quit");
    let status_widget =
        Paragraph::new(status_message).block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(status_widget, area);
}

fn draw_main_content<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    match app.mode {
        AppMode::MainMenu => draw_main_menu(f, app, area),
        AppMode::Help => draw_help(f, area),
        _ => match app.section {
            AppSection::Notes => draw_notes_section(f, app, area),
            AppSection::Todos => draw_todos_section(f, app, area),
            _ => {}
        },
    }
}

fn draw_command_line<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    if let AppMode::Command | AppMode::Renaming = app.mode {
        let command_text = format!(":{}", app.command_buffer);
        let command_widget =
            Paragraph::new(command_text).style(Style::default().fg(Color::Yellow));
        f.render_widget(command_widget, area);
    }
}

fn draw_main_menu<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let menu_items = &["Notes", "Todos", "Help"];
    let items: Vec<ListItem> = menu_items
        .iter()
        .map(|&item| ListItem::new(item))
        .collect();

    let menu = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Main Menu"))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        );

    let mut list_state = ListState::default();
    list_state.select(Some(app.selected_menu_item));
    f.render_stateful_widget(menu, area, &mut list_state);
}

fn draw_notes_section<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(area);

    draw_notes_list(f, app, chunks[0]);
    draw_note_editor(f, app, chunks[1]);
}

fn draw_notes_list<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .notes
        .iter()
        .map(|note| {
            let tag_info = if !note.tags.is_empty() {
                format!(" [{}]", note.tags.join(", "))
            } else {
                String::new()
            };
            ListItem::new(format!("{}{}", note.title, tag_info))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Notes"))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        );

    let mut list_state = ListState::default();
    list_state.select(app.selected_note);
    f.render_stateful_widget(list, area, &mut list_state);
}

fn draw_note_editor<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let editor_block = Block::default().borders(Borders::ALL).title("Editor");
    let inner_area = editor_block.inner(area);
    f.render_widget(editor_block, area);

    if let Some(note) = app.current_note.and_then(|i| app.notes.get(i)) {
        let header = format!(
            "Title: {}\nCreated: {}\nTags: {}\n\n",
            note.title,
            note.created_at,
            if note.tags.is_empty() {
                "None".to_string()
            } else {
                note.tags.join(", ")
            }
        );
        let content = format!("{}{}", header, note.content);
        let editor_text = Paragraph::new(content).wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(editor_text, inner_area);
    }
}

fn draw_todos_section<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(area);

    draw_todos_list(f, app, chunks[0]);
    draw_todo_editor(f, app, chunks[1]);
}

fn draw_todos_list<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .todos
        .iter()
        .map(|todo| {
            let status = if todo.completed { "[✓]" } else { "[ ]" };
            let style = if todo.completed {
                Style::default().fg(Color::Green)
            } else if todo.is_overdue() {
                Style::default().fg(Color::Red)
            } else {
                Style::default()
            };
            let severity = match todo.severity {
                crate::todo::Severity::Critical => "!!!",
                crate::todo::Severity::High => "!!",
                crate::todo::Severity::Medium => "!",
                _ => "",
            };
            ListItem::new(format!("{} {} {}", status, severity, todo.title)).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Todos"))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        );

    let mut list_state = ListState::default();
    list_state.select(app.selected_todo);
    f.render_stateful_widget(list, area, &mut list_state);
}

fn draw_todo_editor<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let editor_block = Block::default().borders(Borders::ALL).title("Editor");
    let inner_area = editor_block.inner(area);
    f.render_widget(editor_block, area);

    if let Some(todo) = app.current_todo.and_then(|i| app.todos.get(i)) {
        let status = if todo.completed {
            "Completed"
        } else if todo.is_overdue() {
            "OVERDUE"
        } else {
            "Pending"
        };
        let due_date = todo
            .due_date
            .as_deref()
            .map_or("Not set", |d| d);
        let tags = if todo.tags.is_empty() {
            "None".to_string()
        } else {
            todo.tags.join(", ")
        };
        let header = format!(
            "Title: {}\nCreated: {}\nStatus: {}\nDue: {}\nSeverity: {}\nTags: {}\n\n",
            todo.title, todo.created_at, status, due_date, todo.severity, tags
        );
        let content = format!("{}{}", header, todo.content);
        let editor_text = Paragraph::new(content);
        f.render_widget(editor_text, inner_area);
    }
}

fn draw_help<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let help_text = "
    Terminal Notes Help
    -------------------

    Navigation:
      j/Down - Move down in list
      k/Up   - Move up in list
      Enter  - Select item/Edit
      Esc    - Go back/Exit editing

    Commands (press : to enter command mode):
      [n]nn    - Create [n] new notes
      [n]ntodo - Create [n] new todos
      [n]del   - Delete [n] items
      :mm      - Go to main menu
      :?       - Show this help
      :save/:w - Save all data
      :rnm     - Rename current note/todo
      :backup  - Create a backup
      :export-md - Export to Markdown
      :export-csv - Export to CSV
      :q/:quit - Quit application
      :q!      - Force quit

    Todo Management:
      Space - Toggle todo completion

    Press Esc to exit this help screen.";

    let help_paragraph = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(help_paragraph, area);
}