use ratatui::{
    Frame, layout::{Constraint, HorizontalAlignment, Layout}, text::{Line, Text}, widgets::{Block, Borders, Paragraph, canvas::Label}
};

use crate::screens::screen::Screen;

pub struct PlayerScreen {}

impl Screen for PlayerScreen {
    fn draw(&self, frame: &mut Frame, area: ratatui::prelude::Rect) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ]);
        let [top, middle, bottom] = vertical.areas(area);
        let horizontal = Layout::horizontal([Constraint::Fill(1); 2]);
        let [left, right] = horizontal.areas(middle);
        
        // frame.render_widget(Block::bordered().borders(Borders::TOP).title("Top"), top);
        // frame.render_widget(Block::bordered().borders(Borders::BOTTOM).title("Bottom"), bottom);
        
        frame.render_widget(Text::from("Top Left").alignment(HorizontalAlignment::Left), top);
        frame.render_widget(Text::from("Top Right").alignment(HorizontalAlignment::Right), top);
        frame.render_widget(Text::from("Bottom Left").alignment(HorizontalAlignment::Left), bottom);
        frame.render_widget(Text::from("Bottom Right").alignment(HorizontalAlignment::Right), bottom);

        frame.render_widget(Block::bordered().title("Left"), left);
        frame.render_widget(Block::bordered().title("Right"), right);

        // frame.render_widget(
        //     // ratatui::widgets::Gauge::default().block(Block::bordered().title("Gauge")).ratio(0.4),
        //     ratatui::widgets::LineGauge::default()
        //         .ratio(0.4)
        //         .filled_symbol("#"),
        //     area,
        // );
    }
}
