use ratatui::{Frame, widgets::Block};

use crate::screens::screen::Screen;

pub struct PlayerScreen {}

impl Screen for PlayerScreen {
    fn draw(&self, frame: &mut Frame, area: ratatui::prelude::Rect) {
        frame.render_widget(
            // ratatui::widgets::Gauge::default().block(Block::bordered().title("Gauge")).ratio(0.4),
            ratatui::widgets::LineGauge::default().block(Block::bordered().title("Line Gauge")).ratio(0.4),
            area,
        );
    }
}
