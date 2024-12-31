use eframe::Error;
mod pages;
mod ui;


enum Page {
    Home,
    RandomNumber,
    RandomString,
}


fn main() -> Result<(), Error> {
    eframe::run_native(
        "Random Generator",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(ui::RandomApp::default())),
    )
}
