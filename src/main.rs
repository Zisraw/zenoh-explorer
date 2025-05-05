use eframe::{egui, NativeOptions, egui::ViewportBuilder};
use std::env;

mod ui;
mod topic_node;

fn main() -> Result<(), eframe::Error> {

    let args: Vec<String> = env::args().collect();
    let config_path = args.get(1).map(|s| s.as_str());

    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size(egui::vec2(400.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Zenoh Explorer",
        options,
        Box::new(|_cc| Ok(Box::new(crate::ui::App::new(config_path)) as Box<dyn eframe::App>)))
    }