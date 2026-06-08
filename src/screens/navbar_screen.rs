use ratatui::layout::{Constraint, Layout};

use crate::{application::action::Action, components::component::Component, screens::screen::Screen};

pub struct Navbar {
    // Dynamic list of components managed by this screen.
    components: Vec<Box<dyn Component>>,
    focused_component: Option<usize>,
}

impl Default for Navbar {
    fn default() -> Self {
        Self { 
            components: Default::default(),
            focused_component: Default::default(),
         }
    }
}

impl Screen for Navbar {
    fn draw(&self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let vertical = Layout::vertical([Constraint::Fill(1)]);
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
            _ => vec![],
        }
    }
}