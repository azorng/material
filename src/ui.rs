use crate::app::App;
use crate::color_data::{N_COLORS, N_VARIANTS};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Frame,
};

pub fn ui(f: &mut Frame, app: &App) {
    let sq_height = (f.size().height - 5) / (N_VARIANTS as u16);
    let sq_width = f.size().width / (N_COLORS as u16);

    // Place left & top centered starting positions
    let left = (f.size().width - (N_COLORS as u16 * sq_width)) / 2;
    let top = (f.size().height - (N_VARIANTS as u16 * sq_height + 2)) / 2;

    let mut pos = (left, top);

    for color in app.colors {
        pos.1 = top;
        for (i, color_var) in color.iter().enumerate() {
            let bg_color = color_var.1.into();
            let text_color = if i < 5 || i > 9 {
                Color::Black
            } else {
                Color::White
            };
            let style = if app.color_fg {
                Style::default().fg(bg_color)
            } else {
                Style::default().bg(bg_color).fg(text_color)
            };
            f.render_widget(
                Block::default()
                    .style(style)
                    .title(color_var.0)
                    .title_alignment(Alignment::Center),
                Rect::new(pos.0, pos.1, sq_width, sq_height),
            );
            pos.1 += sq_height;
        }
        pos.0 += sq_width;
    }

    // last colors are missing 4 variants
    pos.1 += sq_height * 4;

    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Type the color code to copy. Press Esc to exit.")
            .title_alignment(Alignment::Center),
        Rect::new(left, pos.1, sq_width * N_COLORS as u16, sq_height + 3),
    );

    f.render_widget(
        Block::default()
            .title(String::from(&app.input.to_uppercase()))
            .title_alignment(Alignment::Center),
        Rect::new(left, pos.1 + 1, sq_width * N_COLORS as u16, sq_height),
    );

    f.render_widget(
        Block::default()
            .title(String::from(&app.message))
            .title_alignment(Alignment::Center),
        Rect::new(left, pos.1 + 2, sq_width * N_COLORS as u16, sq_height),
    );
}
