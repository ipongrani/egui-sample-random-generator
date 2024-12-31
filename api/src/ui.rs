use eframe::egui;


use crate::Page;
use crate::pages::{
    render_home_page,
    render_random_number_page,
    render_random_string_page
};


pub struct RandomApp {
    current_page: Page,
    random_number: Option<u32>,
    random_string: Option<String>,
}

impl Default for RandomApp {
    fn default() -> Self {
        Self {
            current_page: Page::Home,
            random_number: None,
            random_string: None,
        }
    }
}

impl eframe::App for RandomApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_page {
                Page::Home => render_home_page(ui, &mut self.current_page),
                Page::RandomNumber => render_random_number_page(ui, &mut self.current_page, &mut self.random_number),
                Page::RandomString => render_random_string_page(ui, &mut self.current_page, &mut self.random_string),
            }
        });
    }
}
