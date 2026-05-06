use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect, Spacing},
    style::Stylize,
    symbols::border,
    widgets::{Block, Padding, Paragraph},
};

use crate::{
    application::action::Action,
    screens::{player_screen, screen::Screen},
};

struct EmptyScreen;
impl Screen for EmptyScreen {
    fn draw(&self, frame: &mut Frame, area: Rect) {
        let [view] = Layout::vertical([Constraint::Fill(1)]).areas(area);

        let block = Block::new();

        let inner = block.inner(view);

        frame.render_widget(block, view);

        frame.render_widget(Paragraph::new("Empty.").centered(), inner);
    }

    fn handle_action(&mut self, action: Action) -> Vec<Action> {
        match action {
            _ => {
                vec![]
            }
        }
    }
}
impl Default for EmptyScreen {
    fn default() -> Self {
        EmptyScreen
    }
}

pub struct Root {
    screen: Box<dyn Screen>,
    player_screen: player_screen::PlayerScreen,
}

impl Default for Root {
    fn default() -> Self {
        Self {
            screen: Box::new(EmptyScreen),
            player_screen: player_screen::PlayerScreen::default(),
        }
    }
}

impl Root {
    pub fn draw(&self, frame: &mut Frame) {
        // Panic prevention
        if frame.area().height <= 5 {
            let too_small = Block::new()
                .padding(Padding::vertical(frame.area().height / 2))
                .title("Terminal too small".bold())
                .title_alignment(ratatui::layout::Alignment::Center);

            frame.render_widget(too_small, frame.area());

            // We return early because the application panics when the height becomes
            // less than the height of the global player.
            return;
        }

        // ┏━━━━━━━━━━━━━━━━━━━━━━━━━━┓
        // ┃         Top View         ┃
        // ┣━━━━━━━━━━━━━━━━━━━━━━━━━━┫
        // ┃       Global Player      ┃
        // ┗━━━━━━━━━━━━━━━━━━━━━━━━━━┛
        // The root of the viewtree.
        let [top, bottom] = Layout::vertical([Constraint::Fill(1), Constraint::Length(5)])
            .spacing(Spacing::Overlap(1))
            .areas(frame.area());

        // Global media player block.
        let player_block = Block::bordered()
            .border_set(border::THICK)
            .merge_borders(ratatui::symbols::merge::MergeStrategy::Exact);

        // Render the block
        frame.render_widget(&player_block, bottom);

        // Render the screen inside the block
        self.player_screen.draw(frame, player_block.inner(bottom));

        // ┏━━━━━━━┳━━━━━━━━━━━━━━━━━━┓
        // ┃  Nav  ┃        Main      ┃
        // ┃  Bar  ┃        View      ┃
        // ┣━━━━━━━┻━━━━━━━━━━━━━━━━━━┫
        // ┃       Global Player      ┃
        // ┗━━━━━━━━━━━━━━━━━━━━━━━━━━┛
        let [navbar, view] = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(5)])
            .spacing(Spacing::Overlap(1))
            .areas(top);

        // Viewport block
        let view_port_block = Block::bordered()
            .border_set(border::THICK)
            .merge_borders(ratatui::symbols::merge::MergeStrategy::Exact);

        // Render the block
        frame.render_widget(&view_port_block, view);

        // Render the screen inside the block
        self.screen.draw(frame, view_port_block.inner(view));

        // Navbar block
        let nav_bar_block = Block::bordered()
            .border_set(border::THICK)
            .merge_borders(ratatui::symbols::merge::MergeStrategy::Exact);

        // Render the block
        frame.render_widget(&nav_bar_block, navbar);

        // Render the screen inside the block
        self.screen.draw(frame, nav_bar_block.inner(navbar));

        // Outer border styling
        let [outer_border] = Layout::vertical([Constraint::Fill(1)]).areas(frame.area());

        // Override all outside borders by rendering last (Focused view conflict?)
        let outer_border_block = Block::bordered()
            .title(" 🍾 Rum Runner Radio 📻 ")
            .border_set(border::THICK)
            .merge_borders(ratatui::symbols::merge::MergeStrategy::Fuzzy);

        frame.render_widget(outer_border_block, outer_border);
    }

    pub fn handle_action(&mut self, action: Action) -> Vec<Action> {
        match action {
            Action::FocusNext => todo!(), // TODO: Next view in screens vec
            Action::FocusPrev => todo!(), // TODO: Previous view in screens vec
            _ => self.screen.handle_action(action),
        }
    }
}
