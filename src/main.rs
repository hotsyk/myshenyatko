mod app;
mod event;
mod profiles;
mod settings;
mod ui;

use std::io;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use app::{App, Message, View};

#[derive(Parser)]
#[command(name = "myshenyatko", about = "macOS mouse/trackpad/cursor/keyboard settings TUI")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage saved profiles
    Profile {
        #[command(subcommand)]
        action: ProfileAction,
    },
    /// Dump all current settings as JSON
    Dump,
}

#[derive(Subcommand)]
enum ProfileAction {
    /// List saved profiles
    List,
    /// Apply a saved profile
    Apply { name: String },
    /// Export a profile as JSON to stdout
    Export { name: String },
    /// Import a profile from a JSON file
    Import { file: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        None => run_tui(),
        Some(Commands::Dump) => cmd_dump(),
        Some(Commands::Profile { action }) => match action {
            ProfileAction::List => cmd_profile_list(),
            ProfileAction::Apply { name } => cmd_profile_apply(&name),
            ProfileAction::Export { name } => cmd_profile_export(&name),
            ProfileAction::Import { file } => cmd_profile_import(&file),
        },
    }
}

fn run_tui() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    while app.running {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        if let Some(evt) = event::poll_event(Duration::from_millis(100))? {
            if let Event::Key(key) = evt {
                if app.view == View::ProfileNameInput {
                    handle_input_key(&mut app, key);
                } else if let Some(msg) = event::map_key(key) {
                    app.update(msg);
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn handle_input_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Enter => app.update(Message::ConfirmInput),
        KeyCode::Esc => app.update(Message::Back),
        KeyCode::Backspace => app.update(Message::Backspace),
        KeyCode::Char(c) => app.update(Message::TypeChar(c)),
        _ => {}
    }
}

fn cmd_dump() -> Result<()> {
    let defs = settings::registry::all_settings();
    let values = settings::reader::read_all(&defs);
    let json = serde_json::to_string_pretty(&values)?;
    println!("{json}");
    Ok(())
}

fn cmd_profile_list() -> Result<()> {
    let names = profiles::storage::list()?;
    if names.is_empty() {
        println!("No saved profiles.");
    } else {
        for name in names {
            println!("  {name}");
        }
    }
    Ok(())
}

fn cmd_profile_apply(name: &str) -> Result<()> {
    let profile = profiles::storage::load(name)
        .context(format!("loading profile '{name}'"))?;
    let defs = settings::registry::all_settings();
    let mut applied = 0;
    let mut errors = Vec::new();

    for (id, value) in &profile.settings {
        if let Some(def) = defs.iter().find(|d| d.id == id) {
            match settings::writer::write_setting(def, value) {
                Ok(()) => applied += 1,
                Err(e) => errors.push(format!("{}: {e}", def.description)),
            }
        }
    }

    println!("Applied {applied} settings from profile '{name}'.");
    if !errors.is_empty() {
        eprintln!("Errors:");
        for e in errors {
            eprintln!("  {e}");
        }
    }
    Ok(())
}

fn cmd_profile_export(name: &str) -> Result<()> {
    let json = profiles::storage::export_json(name)?;
    println!("{json}");
    Ok(())
}

fn cmd_profile_import(file: &str) -> Result<()> {
    let json = std::fs::read_to_string(file)
        .context(format!("reading file '{file}'"))?;
    let profile = profiles::storage::import_json(&json)?;
    println!("Imported profile '{}'.", profile.name);
    Ok(())
}
