mod parse;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};



/// # ControlPanel
///
/// Deriving ControlPanel on a struct generates a control
/// panel pseudo-widget (based on [eframe::egui]) that lets you tweak the fields
/// of the struct in real-time.
///
/// This exposes a method `ui` on the underlying struct that can be passed an
/// `&mut eframe::egui::Ui` to paint the panel to the UI.
///
/// # Note:
/// This can be especially useful if you're implementing
/// an algorithm that has a bunch of tunable params and
/// want to inspect the output by tweaking the parameters
/// in real time.
///
/// # Examples
/// ```no_run
/// use egui_controls::ControlPanel;
///
/// #[derive(Debug, Clone, ControlPanel)]
/// pub struct CirclePackingAlgorithmConfig {
///     /// The radius of the circles to pack.
///     #[control(slider(2. ..= 15.0))]
///     pub radius: f64,
///     /// If circles overlap, then how many should be allowed
///     /// to overlap at most.
///     #[control(slider(0 ..= 20))]
///     pub max_overlap_count: usize,
///     #[control(textbox)]
///     pub circle_label: String
/// }
///
/// impl Default for CirclePackingAlgorithmConfig {
/// fn default() -> Self {
///     Self {
///         radius: 12.0,
///         max_overlap_count: 10,
///         circle_label: "Some text".to_string()
///     }
/// }}
///
/// #[derive(Debug, Clone, Default)]
/// pub struct MyApp {
///     settings: CirclePackingAlgorithmConfig
/// }
///
/// impl eframe::App for MyApp {
///     fn update(&mut self, ctx: &::eframe::egui::Context, frame: &mut ::eframe::Frame) {
///         ::eframe::egui::CentralPanel::default().show(ctx, |ui: &mut ::eframe::egui::Ui| {
///             self.settings.ui(ui);
///             ui.vertical(|ui| {
///                 ui.code(format!("{:#?}", &self.settings));
///             });
///         });
///     }
/// }
/// ```
#[proc_macro_derive(ControlPanel, attributes(control))]
pub fn derive(input: TokenStream) -> TokenStream {
    parse::expand(parse_macro_input!(input as DeriveInput))
}
