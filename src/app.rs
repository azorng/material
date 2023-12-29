use crate::color_data::{Colors, COLOR_DATA};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

pub struct App {
    pub message: String,
    pub input: String,
    pub colors: &'static Colors,
}

impl App {
    pub fn new() -> Self {
        App {
            message: String::new(),
            input: String::new(),
            colors: &COLOR_DATA,
        }
    }

    pub fn handle_input(&mut self, c: char) {
        if self.input.len() == 2 {
            self.input = String::new();
        }
        self.input.push(c);
        if self.input.len() == 2 {
            self.color_search();
        }
    }

    pub fn delete_input(&mut self) {
        self.input.pop();
    }

    fn color_search(&mut self) {
        self.colors.iter().for_each(|color| {
            color.iter().for_each(|color_variant| {
                if color_variant.0.to_lowercase() == self.input.to_lowercase() {
                    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                    ctx.set_contents(color_variant.1.to_string()).unwrap();
                    self.message = format!("{} copied to clipboard!", color_variant.1);
                }
            })
        });
    }
}
