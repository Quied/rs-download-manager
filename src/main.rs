use eframe::egui;
use webbrowser;

use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

mod Update;
mod View;

use Update::update::{check_rel_list, check_status, replace_itself};

use egui::widgets::ProgressBar;
use View::view::{show_buttons, show_sys_info};
use std::fs;
// use std::env::consts::EXE_EXTENSION;

// #[derive(Default)]
struct MyApp {
    allowed_to_close: bool,
    allow_to_download: bool,
    available_version: String,

    update_list: Vec<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            allow_to_download: true,
            allowed_to_close: true,
            available_version: String::from("0.1.0"),
            update_list: Vec::new(),
        }
    }
}

fn main() {
    // Window
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(820.0, 540.0)),
        ..Default::default()
    };
    // Window Title
    eframe::run_native(
        "Achernar",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        let APP_VERSION = "Current version 0.1.0".to_owned();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("Check updates")).clicked() {
                    check_status(&mut self.available_version);
                }

                if ui.add(egui::Button::new("Donwload updates")).clicked() {
                    // println!("")
                }

                if ui.add(egui::Button::new("Realeses")).clicked(){
                    check_rel_list();
                }

                if ui.add(egui::Button::new("Replace")).clicked(){
                    replace_itself();
                }
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.add(egui::Label::new("List updates:"));

                ui.separator();
                ui.add(egui::Label::new(APP_VERSION));
            });

            let new_release = "New: ".to_owned() + &self.available_version;

            ui.add(egui::Label::new(new_release));
        });

     
    }
}
