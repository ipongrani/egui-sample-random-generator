use crate::Page;
use eframe::egui;
use core::number::generate_random_number;


pub fn render_random_number_page(
    ui: &mut egui::Ui,
    current_page: &mut Page,
    random_number: &mut Option<u32>,
) {
    ui.heading("Random Number Page");

    ui.separator();

    ui.horizontal(|ui| {
        if ui.button("Back to Home").clicked() {
            *current_page = Page::Home;
            *random_number = None;
        }
    
        if ui.button("Generate Random Number").clicked() {
            *random_number = Some(generate_random_number(1, 100));
        }
    });

    ui.vertical(|ui| {
        if let Some(number) = *random_number {
            ui.label(format!("Generated Number: {}", number));
        }
    });

}
