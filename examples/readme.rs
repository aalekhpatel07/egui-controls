//! This is the example from README.md
//! and it demonstrates the usage of `ControlPanel`
//! derivable proc-macro along with
//! the `#[control]` field attributes.
//!
//! In this example we'll generate a control panel
//! UI for a [CirclePackingAlgorithmConfig] object
//! that exposes the following fields:
//!     - `radius: f64` is controllable by a slider
//!         and has range `2.0 ..= 15.0`,
//!     - `max_overlap_count: usize` is controllable by a slider
//!         and has range `0 ..= 20`,
//!     - `circle_label: String` is controllable as a textbox,
//!     - `non_changing_global_value: i8` is a non-interactive field.

use egui_controls::ControlPanel;

/// Some config data for the
#[derive(Debug, Clone, ControlPanel)]
pub struct CirclePackingAlgorithmConfig {
    /// The radius of the circles to pack.
    #[control(slider(2.0 ..= 15.0))]
    pub radius: f64,
    /// If circles overlap, then how many should be allowed
    /// to overlap at most.
    #[control(slider(0 ..= 20))]
    pub max_overlap_count: usize,
    /// Once we find the circles, label them with the
    /// given name.
    #[control(textbox)]
    pub circle_label: String,
    /// Some global constant that should definitely only take on this value.
    pub non_changing_global_value: i8,
}

/// Some initial values for the config that make sense.
impl Default for CirclePackingAlgorithmConfig {
    fn default() -> Self {
        Self {
            radius: 12.0,
            max_overlap_count: 10,
            circle_label: "Some text".to_string(),
            non_changing_global_value: 42,
        }
    }
}

use eframe::{egui, Frame};

#[derive(Debug, Clone, Default)]
pub struct MyApp {
    settings: CirclePackingAlgorithmConfig,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            // Embed the settings panel
            // directly into your ui.
            self.settings.ui(ui);
            // Add this the struct's debug repr if you want
            // to see the values getting updated as you tweak
            // the settings via the ui.
            ui.vertical(|ui| {
                ui.code(format!("{:#?}", &self.settings));
            });
        });
    }
}

// Write the usual eframe entrypoint.
pub fn main() {
    let options = ::eframe::NativeOptions {
        resizable: true,
        initial_window_size: Some(::eframe::egui::vec2(2000.0, 500.0)),
        ..Default::default()
    };
    let app = MyApp::default();
    ::eframe::run_native("readme", options, Box::new(|_| Box::new(app))).unwrap();
}
