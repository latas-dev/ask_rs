use eframe::egui;

#[derive(Default)]
pub struct MyEguiApp {}

impl MyEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        // setup 

        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Keyboard");
        });
    }
}