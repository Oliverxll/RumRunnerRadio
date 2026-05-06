use ratatui::{Frame, layout::Rect};

use crate::application::action::Action;

pub trait Screen {
    fn draw(&self, frame: &mut Frame, area: Rect);
    fn handle_action(&mut self, action: Action) -> Vec<Action>;
}
