use crate::application::action::Action;

pub trait Component {
    fn set_focused(&mut self, focused: bool);
    fn handle_action(&mut self, action: Action) -> Vec<Action>;
}
