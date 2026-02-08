mod cursor;
mod diff;
mod keyboard;
mod mouse;
mod profiles;
mod scroll;
mod trackpad;
mod widgets;

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};

use crate::app::{App, View};
use crate::settings::Tab;

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(frame.area());

    draw_tab_bar(frame, app, chunks[0]);

    match app.view {
        View::Settings => draw_settings(frame, app, chunks[1]),
        View::Review => diff::draw(frame, app, chunks[1]),
        View::Profiles => profiles::draw(frame, app, chunks[1]),
        View::ProfileNameInput => profiles::draw_name_input(frame, app, chunks[1]),
    }

    draw_status_bar(frame, app, chunks[2]);
}

fn draw_tab_bar(frame: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = Tab::ALL
        .iter()
        .map(|t| {
            let style = if *t == app.tab {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            Line::from(Span::styled(t.label(), style))
        })
        .collect();

    let idx = Tab::ALL.iter().position(|t| *t == app.tab).unwrap_or(0);
    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" myshenyatko "),
        )
        .select(idx)
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(tabs, area);
}

fn draw_settings(frame: &mut Frame, app: &App, area: Rect) {
    match app.tab {
        Tab::Mouse => mouse::draw(frame, app, area),
        Tab::Trackpad => trackpad::draw(frame, app, area),
        Tab::ScrollWindow => scroll::draw(frame, app, area),
        Tab::Cursor => cursor::draw(frame, app, area),
        Tab::Keyboard => keyboard::draw(frame, app, area),
    }
}

fn draw_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let pending = app.pending_change_count();
    let status_text = if let Some(ref msg) = app.status_message {
        msg.clone()
    } else if pending > 0 {
        format!("{pending} changes pending")
    } else {
        String::new()
    };

    let keybinds = match app.view {
        View::Settings => "[Tab] switch  [↑↓] navigate  [←→] adjust  [Space] toggle  [r]eview  [p]rofiles  [q]uit",
        View::Review => "[a]pply  [c]ancel  [s]ave profile  [Esc] back",
        View::Profiles => "[↑↓] select  [Enter] apply  [d]elete  [Esc] back",
        View::ProfileNameInput => "[Enter] confirm  [Esc] cancel",
    };

    let bar = Paragraph::new(Line::from(vec![
        Span::styled(&status_text, Style::default().fg(Color::Green)),
        Span::raw("  "),
        Span::styled(keybinds, Style::default().fg(Color::DarkGray)),
    ]))
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(bar, area);
}
