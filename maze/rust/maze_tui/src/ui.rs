use ratatui::Frame;

use crate::app::App;

pub fn render(app: &App, frame: &mut Frame) {
    frame.render_widget(app, frame.area());
}
