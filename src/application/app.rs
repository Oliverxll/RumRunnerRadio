use std::{collections::VecDeque, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::{application::action::Action, screens::root::Root};

#[derive(Default)]
pub struct App {
    exit: bool,
    root: Root,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        const TICK_RATE: Duration = Duration::from_millis(200);

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            // Event reads block until an event is read. Event::poll is used to set aa timeout for these reading to not block the application.
            // It would probably be better to listen for events in a separate thread to have a non-blocking main thread.
            if event::poll(TICK_RATE)? {
                let event = event::read()?;
                let actions = self.handle_events(&event);
                self.dispatch(actions);
            } else {
                self.dispatch(vec![Action::Tick]);
            }
        }

        Ok(())
    }

    fn handle_events(&mut self, event: &Event) -> Vec<Action> {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => vec![],
        }
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent) -> Vec<Action> {
        match key_event.code {
            KeyCode::Esc => vec![Action::Exit],
            _ => vec![],
        }
    }

    fn dispatch(&mut self, actions: Vec<Action>) {
        let mut queue: VecDeque<Action> = actions.into();

        while let Some(action) = queue.pop_front() {
            match action {
                Action::Exit => {
                    self.exit = true;
                }
                Action::FocusNext | Action::FocusPrev => {}
                _ => {}
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        self.root.draw(frame);
    }
}
