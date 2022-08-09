use crate::app::App;
use crate::color_data::{N_COLORS, N_VARIANTS};
use colors_transform::{Color as _Color, Rgb};
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let sq_height = f.size().height / (N_VARIANTS as u16 + 5);
    let sq_width = f.size().width / (N_COLORS as u16);

    // Place left & top centered starting positions
    let left = (f.size().width - (N_COLORS as u16 * sq_width)) / 2;
    let top = (f.size().height - (N_VARIANTS as u16 * sq_height + 2)) / 2;

    let mut pos = (left, top);

    for color in app.colors {
        pos.1 = top;
        for (i, color_var) in color.iter().enumerate() {
            let rgb = Rgb::from_hex_str(color_var.1).unwrap().as_tuple();
            let bg_color = Color::Rgb(rgb.0 as u8, rgb.1 as u8, rgb.2 as u8);
            let text_color = if i < 5 { Color::Black } else { Color::White };
            f.render_widget(
                Block::default()
                    .style(Style::default().bg(bg_color).fg(text_color))
                    .title(color_var.0)
                    .title_alignment(Alignment::Center),
                Rect::new(pos.0, pos.1, sq_width, sq_height),
            );
            pos.1 += sq_height;
        }
        pos.0 += sq_width;
    }

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
