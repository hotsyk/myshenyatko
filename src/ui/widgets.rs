use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::settings::{Constraint, FloatRange, SettingDef, SettingValue};

pub fn render_setting_row(
    frame: &mut Frame,
    area: Rect,
    def: &SettingDef,
    value: Option<&SettingValue>,
    is_changed: bool,
    is_selected: bool,
) {
    let label_style = if is_selected {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let value_style = if is_changed {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let cursor = if is_selected { "▸ " } else { "  " };
    let label = format!("{}{:<40}", cursor, def.description);

    let value_display = match value {
        Some(val) => format_value(val, &def.constraint),
        None => "—".to_string(),
    };

    let changed_marker = if is_changed { " *" } else { "" };

    let help_text = if !def.help.is_empty() {
        format!("  {}", def.help)
    } else {
        String::new()
    };

    let line = Line::from(vec![
        Span::styled(label, label_style),
        Span::styled(value_display, value_style),
        Span::styled(changed_marker, Style::default().fg(Color::Yellow)),
        Span::styled(help_text, Style::default().fg(Color::DarkGray)),
    ]);

    frame.render_widget(Paragraph::new(line), area);
}

fn format_value(val: &SettingValue, constraint: &Constraint) -> String {
    match (val, constraint) {
        (SettingValue::Float(v), Constraint::FloatRange(FloatRange { min, max, .. })) => {
            let bar = render_slider(*v, *min, *max, 20);
            format!("{bar} {v:.2}")
        }
        (SettingValue::Bool(v), _) => {
            if *v {
                "[x] On".to_string()
            } else {
                "[ ] Off".to_string()
            }
        }
        (SettingValue::Int(v), _) => format!("{v}"),
        (SettingValue::Str(v), _) => v.clone(),
        _ => val.to_string(),
    }
}

fn render_slider(value: f64, min: f64, max: f64, width: usize) -> String {
    let range = max - min;
    if range <= 0.0 {
        return "=".repeat(width);
    }
    let pos = ((value - min) / range * width as f64).round() as usize;
    let pos = pos.min(width);
    let mut bar = String::with_capacity(width + 2);
    bar.push('[');
    for i in 0..width {
        if i == pos {
            bar.push('●');
        } else {
            bar.push('═');
        }
    }
    bar.push(']');
    bar
}

pub fn render_group_header(frame: &mut Frame, area: Rect, title: &str) {
    let line = Line::from(Span::styled(
        format!("── {title} ──"),
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    frame.render_widget(Paragraph::new(line), area);
}
