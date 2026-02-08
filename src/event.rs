use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

use crate::app::Message;

pub fn poll_event(timeout: Duration) -> Result<Option<Event>> {
    if event::poll(timeout)? {
        Ok(Some(event::read()?))
    } else {
        Ok(None)
    }
}

pub fn map_key(key: KeyEvent) -> Option<Message> {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return Some(Message::Quit);
    }

    match key.code {
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Tab => Some(Message::NextTab),
        KeyCode::BackTab => Some(Message::PrevTab),
        KeyCode::Up | KeyCode::Char('k') => Some(Message::NavigateUp),
        KeyCode::Down | KeyCode::Char('j') => Some(Message::NavigateDown),
        KeyCode::Left | KeyCode::Char('h') => Some(Message::AdjustLeft),
        KeyCode::Right | KeyCode::Char('l') => Some(Message::AdjustRight),
        KeyCode::Char(' ') | KeyCode::Enter => Some(Message::Toggle),
        KeyCode::Char('r') => Some(Message::OpenReview),
        KeyCode::Char('a') => Some(Message::ApplyChanges),
        KeyCode::Char('c') => Some(Message::CancelReview),
        KeyCode::Char('s') => Some(Message::SaveProfile),
        KeyCode::Char('p') => Some(Message::OpenProfiles),
        KeyCode::Char('d') => Some(Message::DeleteProfile),
        KeyCode::Esc => Some(Message::Back),
        _ => None,
    }
}
