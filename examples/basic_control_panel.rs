use egui_controls::ControlPanel;
use serde::{Deserialize, Serialize};

/// Some data that represents tunable settings for some algorithm.
#[derive(Default, Debug, Clone, Serialize, Deserialize, ControlPanel)]
pub struct Settings {
    /// Some description for alpha. This could be really long or short.
    /// Multi-line docstrings are fine too. This one's an example of
    /// a really really really really really really really really
    /// long string.
    #[control(slider(0.0..=1.0))]
    pub alpha: f64,
    /// Some description for beta. This could be really long or short.
    #[control(slider(2 ..= 10))]
    pub beta: usize,
    /// Gamma is some magical text.
    #[control(textbox)]
    pub gamma: String,
}

#[derive(Default)]
pub struct MyApp {
    pub settings: Settings,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &::eframe::egui::Context, _frame: &mut ::eframe::Frame) {
        ::eframe::egui::CentralPanel::default()
            .show(ctx, |ui: &mut ::eframe::egui::Ui| {
            ui.horizontal(|ui| {

                self.settings.ui(ui);
            });
            ui.vertical(|ui| {
                ui.code(format!("{:#?}", &self.settings));
            });
        });
    }
}

fn main() {
    let options = ::eframe::NativeOptions {
        resizable: true,
        initial_window_size: Some(::eframe::egui::vec2(2000.0, 500.0)),
        ..Default::default()
    };
    let app = MyApp::default();
    ::eframe::run_native("control_panel", options, Box::new(|_| Box::new(app)))
        .expect("run native to work");
}
