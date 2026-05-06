use crate::{application::action::Action, screens::screen::Screen};

pub struct Navbar {

}

impl Screen for Navbar {
    fn draw(&self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        todo!()
    }
    
    fn handle_action(&mut self, action: Action) -> Vec<Action> {
        match action {
            Action::NavigateUp => {
                print!("Navigating up in Navbar");
                vec![]
            }
            Action::NavigateDown => {
                print!("Navigating down in Navbar");
                vec![]
            }
            Action::NavigateLeft => {
                print!("Navigating left in Navbar");
                vec![]
            }
            Action::NavigateRight => {
                print!("Navigating right in Navbar");
                vec![]
            }
            _ => todo!(),
        }
    }
}