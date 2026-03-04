use ratatui::{Frame, layout::Rect};

pub trait Screen {
    fn draw(&self, frame: &mut Frame, area: Rect);
}
