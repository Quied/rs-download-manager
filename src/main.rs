use eframe::egui;

mod Update;
mod View;

use Update::update::{check_updates, replace_itself, self_delete, server_reqwest};

// #[derive(Default)]
struct MyApp {
    current_version: String
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            current_version: "v0.1.0".to_owned(),
        }
    }
}

fn main() {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 240.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "rs-downloader",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
    .expect("msg");
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {

                ui.add(egui::Label::new(&self.current_version));
                ui.separator();

                if ui.add(egui::Button::new("Check updates")).clicked() {
                    let _ = check_updates();
                }

                if ui.add(egui::Button::new("Replace itself")).clicked() {
                    let _ = replace_itself();
                }

                if ui.add(egui::Button::new("Self delete")).clicked() {
                    let _ = self_delete();
                }

                if ui.add(egui::Button::new("Server Reqwest")).clicked() {
                    let url = format!("http://localhost:3002/local");

                    let _ = server_reqwest(url);
                }
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.add(egui::Label::new("List updates:"));
            });
        });
    }
}
