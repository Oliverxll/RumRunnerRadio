use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect, Spacing},
    style::Stylize,
    symbols::border,
    widgets::{Block, Padding, Paragraph},
};

use crate::screens::screen::Screen;

struct EmptyScreen;
impl Screen for EmptyScreen {
    fn draw(&self, frame: &mut Frame, area: Rect) {
        let [view] = Layout::vertical([Constraint::Fill(1)]).areas(area);

        let block = Block::bordered();

        let inner = block.inner(view);

        frame.render_widget(block, view);

        frame.render_widget(Paragraph::new("Empty.").centered(), inner);
    }
}
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
        if frame.area().height < 6 {
            let too_small = Block::new()
                .padding(Padding::vertical(frame.area().height / 2))
                .title("Terminal too small".bold())
                .title_alignment(ratatui::layout::Alignment::Center);

            frame.render_widget(too_small, frame.area());

            // We return early because the application panics when the height becomes 5 < from the defined layout.
            return;
        }

        let [top, bottom] = Layout::vertical([Constraint::Fill(1), Constraint::Length(5)])
            .spacing(Spacing::Overlap(1))
            .areas(frame.area());

        // The root of the viewtree.
        let top_block =
            Block::bordered().merge_borders(ratatui::symbols::merge::MergeStrategy::Exact);

        let inner_top_block = top_block.inner(top);

        frame.render_widget(top_block, top);

        // Global media player.
        let bottom_block =
            Block::bordered().merge_borders(ratatui::symbols::merge::MergeStrategy::Exact);

        let inner_bottom_block = bottom_block.inner(bottom);

        frame.render_widget(bottom_block, bottom);

        // Outer border styling.
        let [outer_border] = Layout::vertical([Constraint::Fill(1)]).areas(frame.area());

        let outer_border_block = Block::bordered()
            .title(" 🍾 Rum Runner Radio 📻 ")
            .border_set(border::THICK)
            .merge_borders(ratatui::symbols::merge::MergeStrategy::Replace);
        frame.render_widget(outer_border_block, outer_border);

        // Pass the inner area for rendering to avoid overdrawing previous rendering.
        self.screen.draw(frame, inner_top_block);
    }
}
