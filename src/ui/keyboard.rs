use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Block, Borders};

use crate::app::App;
use super::widgets::{render_group_header, render_setting_row};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title(" Keyboard ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let visible = app.visible_settings();
    let row_count = visible.len() + 2;
    let constraints: Vec<Constraint> = (0..row_count)
        .map(|_| Constraint::Length(1))
        .chain(std::iter::once(Constraint::Min(0)))
        .collect();

    let rows = Layout::default()
        .constraints(constraints)
        .split(inner);

    let mut row_idx = 0;
    let mut setting_idx = 0;
    let mut last_group = None;

    for def in &visible {
        if last_group != Some(def.group) {
            if row_idx < rows.len() {
                render_group_header(frame, rows[row_idx], &def.group.to_string());
                row_idx += 1;
            }
            last_group = Some(def.group);
        }

        if row_idx < rows.len() {
            let is_changed = app.pending_changes.contains_key(def.id);
            let value = app.effective_value(def.id);
            render_setting_row(frame, rows[row_idx], def, value, is_changed, setting_idx == app.selected_row);
            row_idx += 1;
        }
        setting_idx += 1;
    }
}
