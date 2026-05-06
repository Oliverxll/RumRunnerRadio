use crate::{application::action::Action, screens::screen::Screen};

pub struct HomeScreen {}

impl Screen for HomeScreen {
    fn draw(&self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        todo!()
    }

    fn handle_action(&mut self, action: Action) -> Vec<Action> {
        match action {
            Action::NavigateUp => {
                print!("Navigating up in HomeScreen");
                vec![]
            }
            Action::NavigateDown => {
                print!("Navigating down in HomeScreen");
                vec![]
            }
            Action::NavigateLeft => {
                print!("Navigating left in HomeScreen");
                vec![]
            }
            Action::NavigateRight => {
                print!("Navigating right in HomeScreen");
                vec![]
            }
            _ => todo!(),
        }
    }
}
