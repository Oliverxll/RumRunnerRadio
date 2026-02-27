use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    widgets::Block,
};

use crate::screens::screen::Screen;

struct EmptyScreen;
impl Screen for EmptyScreen {}
impl Default for EmptyScreen {
    fn default() -> Self {
        EmptyScreen
    }
}

pub struct Root {
    screen: Box<dyn Screen>,
}

impl Default for Root {
    fn default() -> Self {
        Self {
            screen: Box::new(EmptyScreen),
        }
    }
}

impl Root {
    pub fn draw(&self, frame: &mut Frame) {
        let [top, bottom] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(5)]).areas(frame.area());

        let top_block = Block::bordered();
        frame.render_widget(top_block, top);

        let bottom_block = Block::bordered();
        frame.render_widget(bottom_block, bottom);
    }
}
