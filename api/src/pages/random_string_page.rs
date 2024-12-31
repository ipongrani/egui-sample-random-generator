use crate::Page;
use eframe::egui;
use core::string::generate_random_string;

pub fn render_random_string_page(
    ui: &mut egui::Ui,
    current_page: &mut Page,
    random_string: &mut Option<String>,
) {
    ui.heading("Random String Page");
    
    ui.separator();

    if ui.button("Back to Home").clicked() {
        *current_page = Page::Home;
    }
   
    if ui.button("Generate Random String").clicked() {
        *random_string = Some(generate_random_string(10));
    }

    if let Some(random_str) = random_string {
        ui.label(format!("Generated String: {}", random_str));
    }

    
}


