pub mod dialogs;
pub mod theme;

use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout},
    style::{Style, Stylize},
    text::Line,
    widgets::Block,
};

use crate::app::App;

pub fn draw(f: &mut Frame, app: &App) {
    let area = f.area();
    let t = &app.theme;

    f.render_widget(Block::new().style(Style::new().bg(t.bg).fg(t.text)), area);

    let [row] = Layout::vertical([Constraint::Length(1)])
        .flex(Flex::Center)
        .areas(area);
    f.render_widget(
        Line::styled("crab", Style::new().fg(t.accent).bold()).centered(),
        row,
    );

    if app.help_open {
        dialogs::help(f, area, t);
    }
}
