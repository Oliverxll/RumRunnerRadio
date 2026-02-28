use ratatui::{
    Frame,
    layout::{Constraint, Layout, Spacing},
    symbols::border,
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
        let [top, bottom] = Layout::vertical([Constraint::Fill(1), Constraint::Length(5)])
            .spacing(Spacing::Overlap(1))
            .areas(frame.area());

        // The root of the viewtree.
        let top_block =
            Block::bordered().merge_borders(ratatui::symbols::merge::MergeStrategy::Exact);
        frame.render_widget(top_block, top);

        // Global media player.
        let bottom_block =
            Block::bordered().merge_borders(ratatui::symbols::merge::MergeStrategy::Exact);
        frame.render_widget(bottom_block, bottom);

        // Outer border styling.
        let [outer_border] = Layout::vertical([Constraint::Fill(1)]).areas(frame.area());
        let outer_border_block = Block::bordered()
            .title(" 🍾 Rum Runner Radio 📻 ")
            .border_set(border::THICK)
            .merge_borders(ratatui::symbols::merge::MergeStrategy::Replace);
        frame.render_widget(outer_border_block, outer_border);
    }
}
