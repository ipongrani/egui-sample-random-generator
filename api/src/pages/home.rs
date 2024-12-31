use crate::Page;
use eframe::egui;


pub fn render_home_page(ui: &mut egui::Ui, current_page: &mut Page) {
    ui.heading("Home Page");

    ui.separator();
    
    if ui.button("Go to Random Number Page").clicked() {
        *current_page = Page::RandomNumber;
    }
    if ui.button("Go to Random String Page").clicked() {
        *current_page = Page::RandomString;
    }
}