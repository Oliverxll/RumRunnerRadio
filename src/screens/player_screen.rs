use ratatui::{
    Frame,
    layout::{Constraint, HorizontalAlignment, Layout},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, canvas::Label},
};

use crate::{
    application::action::Action, components::component::Component, screens::screen::Screen,
};

pub struct PlayerScreen {
    // Dynamic list of components managed by this screen.
    components: Vec<Box<dyn Component>>,
    focused_component: Option<usize>,
}

impl Default for PlayerScreen {
    fn default() -> Self {
        Self {
            components: Default::default(),
            focused_component: Default::default(),
        }
    }
}

impl Screen for PlayerScreen {
    fn draw(&self, frame: &mut Frame, area: ratatui::prelude::Rect) {
        // Following commented code produces:
        // ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
        // ┃Top Left                 Top Right┃
        // ┃ 40% ###########──────────────────┃
        // ┃Bottom Left           Bottom Right┃
        // ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ]);
        let [top, middle, bottom] = vertical.areas(area);

        frame.render_widget(
            Text::from("Top Left").alignment(HorizontalAlignment::Left),
            top,
        );
        frame.render_widget(
            Text::from("Top Right").alignment(HorizontalAlignment::Right),
            top,
        );
        frame.render_widget(
            Text::from("Bottom Left").alignment(HorizontalAlignment::Left),
            bottom,
        );
        frame.render_widget(
            Text::from("Bottom Right").alignment(HorizontalAlignment::Right),
            bottom,
        );

        // ratatui::widgets::Gauge::default().block(Block::bordered().title("Gauge")).ratio(0.4),
        frame.render_widget(
            ratatui::widgets::LineGauge::default()
                .ratio(0.4)
                .filled_symbol("#"),
            middle,
        );
        // ########################################################################

        // Following comment code produces:
        // ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
        // ┃Left Top                 Right Top┃
        // ┃Left Middle           Right Middle┃
        // ┃Left Bottom           Right Bottom┃
        // ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

        // let horizontal = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(3)]);
        // let [left, right] = horizontal.areas(area);

        // let verticalLeft = Layout::vertical([Constraint::Fill(1); 3]);
        // let verticalRight = Layout::vertical([Constraint::Fill(1); 3]);

        // let [lt, lm, lb] = verticalLeft.areas(area);
        // let [rt, rm, rb] = verticalRight.areas(area);

        // frame.render_widget(Text::from("Left Top"), lt);
        // frame.render_widget(Text::from("Left Middle"), lm);
        // frame.render_widget(Text::from("Left Bottom"), lb);

        // frame.render_widget(Text::from("Right Top").alignment(HorizontalAlignment::Right), rt);
        // frame.render_widget(Text::from("Right Middle").alignment(HorizontalAlignment::Right), rm);
        // frame.render_widget(Text::from("Right Bottom").alignment(HorizontalAlignment::Right), rb);
    }

    fn handle_action(&mut self, action: Action) -> Vec<Action> {
        match action {
            Action::NavigateUp => {
                print!("Navigating up in PlayerScreen");
                vec![]
            }
            Action::NavigateDown => {
                print!("Navigating down in PlayerScreen");
                vec![]
            }
            Action::NavigateLeft => {
                print!("Navigating left in PlayerScreen");
                vec![]
            }
            Action::NavigateRight => {
                print!("Navigating right in PlayerScreen");
                vec![]
            }
            _ => vec![],
        }
    }
}
