use egui_controls::ControlPanel;

use eframe::egui;
use serde::{Serialize, Deserialize};


#[derive(Default, Copy, Clone, Serialize, Deserialize, ControlPanel)]
pub struct Settings {
    /// Some description for alpha. This could be really long or short.
    #[control(slider(min=0.0, max=1.0))]
    pub alpha: f64,
    /// Some description for beta. This could be really long or short.
    #[control(slider(min=0, max=10))]
    pub beta: usize,
}

#[derive(Default)]
pub struct MyApp {
    pub settings: Settings
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
        .show(ctx, |ui: &mut egui::Ui| {
            ui.add(self.settings);
        });
    }
}


fn main() {
    let options = eframe::NativeOptions {
        resizable: true,
        initial_window_size: Some(egui::vec2(800.0, 500.0)),
        ..Default::default()
    };
    let app = MyApp::default();

    eframe::run_native("control_panel", options, Box::new(|_| Box::new(app))).unwrap();
}