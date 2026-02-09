use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Profiles ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if app.profile_names.is_empty() {
        let msg = Paragraph::new("  No saved profiles. Press [n] to create one from current settings.")
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(msg, inner);
        return;
    }

    let constraints: Vec<Constraint> = (0..app.profile_names.len())
        .map(|_| Constraint::Length(1))
        .chain(std::iter::once(Constraint::Min(0)))
        .collect();

    let rows = Layout::default()
        .constraints(constraints)
        .split(inner);

    for (i, name) in app.profile_names.iter().enumerate() {
        if i >= rows.len() {
            break;
        }
        let is_selected = i == app.profile_selected;
        let cursor = if is_selected { "▸ " } else { "  " };
        let style = if is_selected {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let line = Line::from(Span::styled(format!("{cursor}{name}"), style));
        frame.render_widget(Paragraph::new(line), rows[i]);
    }
}

pub fn draw_name_input(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Save Profile ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let constraints = [Constraint::Length(1), Constraint::Length(1), Constraint::Min(0)];
    let rows = Layout::default().constraints(constraints).split(inner);

    let prompt = Paragraph::new("  Enter profile name:")
        .style(Style::default().fg(Color::White));
    frame.render_widget(prompt, rows[0]);

    let input = Paragraph::new(format!("  > {}_", app.input_buffer))
        .style(Style::default().fg(Color::Cyan));
    frame.render_widget(input, rows[1]);
}
