use crossterm::event::KeyEvent;

pub enum Action {
    Exit,
    Tick,
    Key(KeyEvent),
    FocusNext,
    FocusPrev,
    NavigateUp,
    NavigateDown,
    NavigateLeft,
    NavigateRight,
}
