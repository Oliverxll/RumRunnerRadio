use std::time::Duration;

use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};

use crate::application::action::Action;

pub struct App {
    exit: bool,
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
                let actions = self.handle_events(event);

                self.dispatch(actions);
            } else {
                self.dispatch(vec![Action::Tick]);
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        todo!()
    }

    fn handle_events(&mut self, event: Event) -> Vec<Action> {
        todo!()
    }

    fn dispatch(&mut self, actions: Vec<Action>) {}
}
