use crate::application::app::App;

mod application;
mod components;
mod screens;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}
