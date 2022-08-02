use crate::app::App;
use colors_transform::{Color as _Color, Rgb};
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let sq_height = f.size().height / (app.colors[0].variants.len() as u16 + 5);
    let sq_width = f.size().width / (app.colors.len() as u16);

    // Place left & top centered starting positions
    let left = (f.size().width - (app.colors.len() as u16 * sq_width)) / 2;
    let top = (f.size().height - (app.colors[0].variants.len() as u16 * sq_height + 5)) / 2;

    let mut pos = (left, top);

    app.colors.iter().for_each(|color| {
        pos.1 = top;
        color
            .variants
            .iter()
            .enumerate()
            .for_each(|(i, color_var)| {
                let rgb = Rgb::from_hex_str(color_var.1).unwrap().as_tuple();
                let bg_color = Color::Rgb(rgb.0 as u8, rgb.1 as u8, rgb.2 as u8);
                let text_color = if i < 5 { Color::Black } else { Color::White };
                let block = Block::default()
                    .style(Style::default().bg(bg_color).fg(text_color))
                    .title(color_var.0)
                    .title_alignment(Alignment::Center);
                let rect = Rect::new(pos.0, pos.1, sq_width, sq_height);
                f.render_widget(block, rect);
                pos.1 += sq_height;
            });
        pos.0 += sq_width;
    });

    f.render_widget(
        Block::default()
            .style(Style::default())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Type the color code to copy. Press Esc to exit.")
            .title_alignment(Alignment::Center),
        Rect::new(
            left,
            pos.1,
            sq_width * app.colors.len() as u16,
            sq_height + 3,
        ),
    );

    f.render_widget(
        Block::default()
            .style(Style::default())
            .title(String::from(&app.input.to_uppercase()))
            .title_alignment(Alignment::Center),
        Rect::new(
            left,
            pos.1 + 1,
            sq_width * app.colors.len() as u16,
            sq_height,
        ),
    );

    f.render_widget(
        Block::default()
            .style(Style::default())
            .title(String::from(&app.message))
            .title_alignment(Alignment::Center),
        Rect::new(
            left,
            pos.1 + 2,
            sq_width * app.colors.len() as u16,
            sq_height,
        ),
    );
}
