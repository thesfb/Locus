// src/main.rs
mod app;
mod file_io;
mod note;
mod todo;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

use app::{App, AppMode, AppSection};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = match App::new() {
        Ok(app) => app,
        Err(err) => {
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;
            eprintln!("Error initializing app: {}", err);
            return Err(err);
        }
    };

    let res = run_app(&mut terminal, &mut app);

    if app.has_unsaved_changes() {
        if let Err(save_err) = app.save_data() {
            eprintln!("Error saving data on exit: {}", save_err);
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("An error occurred: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = std::time::Instant::now();

    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    if !app.has_unsaved_changes() {
                        break;
                    }
                    app.status_message =
                        Some("Unsaved changes! Use :save or :q! to force quit".to_string());
                } else {
                    match app.mode {
                        AppMode::MainMenu => handle_main_menu_input(app, key),
                        AppMode::Normal => handle_normal_mode_input(app, key),
                        AppMode::Command => handle_command_mode_input(app, key),
                        AppMode::Editing => handle_editing_mode_input(app, key),
                        AppMode::Help => handle_help_mode_input(app, key),
                        AppMode::Renaming => handle_renaming_mode_input(app, key),
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = std::time::Instant::now();
        }
    }
    Ok(())
}

fn handle_main_menu_input(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Char(':') => app.mode = AppMode::Command,
        KeyCode::Char('j') | KeyCode::Down => app.next_menu_item(),
        KeyCode::Char('k') | KeyCode::Up => app.previous_menu_item(),
        KeyCode::Enter => app.select_menu_item(),
        _ => {}
    }
}

fn handle_normal_mode_input(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Char(':') => app.mode = AppMode::Command,
        KeyCode::Char('j') | KeyCode::Down => match app.section {
            AppSection::Notes => app.next_note(),
            AppSection::Todos => app.next_todo(),
            _ => {}
        },
        KeyCode::Char('k') | KeyCode::Up => match app.section {
            AppSection::Notes => app.previous_note(),
            AppSection::Todos => app.previous_todo(),
            _ => {}
        },
        KeyCode::Enter => match app.section {
            AppSection::Notes => {
                if let Some(idx) = app.selected_note {
                    app.current_note = Some(idx);
                    app.mode = AppMode::Editing;
                }
            }
            AppSection::Todos => {
                if let Some(idx) = app.selected_todo {
                    app.current_todo = Some(idx);
                    app.mode = AppMode::Editing;
                }
            }
            _ => {}
        },
        KeyCode::Char(' ') if matches!(app.section, AppSection::Todos) => {
            app.toggle_todo_completion()
        }
        _ => {}
    }
}

fn handle_command_mode_input(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Esc => app.mode = AppMode::Normal,
        KeyCode::Enter => {
            if app
                .status_message
                .as_deref()
                .map_or(false, |msg| msg == "Enter new name:")
            {
                let new_name = app.command_buffer.clone();
                app.finish_rename(new_name);
            } else {
                app.execute_command();
            }
        }
        KeyCode::Char(c) => app.command_buffer.push(c),
        KeyCode::Backspace => {
            app.command_buffer.pop();
        }
        _ => {}
    }
}

fn handle_editing_mode_input(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Esc => app.mode = AppMode::Normal,
        KeyCode::Enter => app.insert_new_line(),
        KeyCode::Char(c) => app.insert_char(c),
        KeyCode::Backspace => app.delete_char(),
        _ => {}
    }
}

fn handle_help_mode_input(app: &mut App, key: event::KeyEvent) {
    if key.code == KeyCode::Esc {
        app.mode = AppMode::Normal;
    }
}

fn handle_renaming_mode_input(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.mode = AppMode::Normal;
            app.status_message = Some("Rename canceled".to_string());
        }
        KeyCode::Enter => {
            let new_name = app.command_buffer.clone();
            app.finish_rename(new_name);
        }
        KeyCode::Char(c) => app.command_buffer.push(c),
        KeyCode::Backspace => {
            app.command_buffer.pop();
        }
        _ => {}
    }
}