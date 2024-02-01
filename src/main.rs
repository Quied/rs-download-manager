use eframe::egui;
use self_update::update::Release;
use webbrowser;

use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

mod Update;
mod View;

use Update::update::{check_rel_list, check_status, get_releases_list, replace_itself};

use egui::widgets::ProgressBar;
use std::fs;
use View::view::{show_buttons, show_sys_info};
// use std::env::consts::EXE_EXTENSION;

// #[derive(Default)]
struct MyApp {
    allowed_to_close: bool,
    allow_to_download: bool,
    available_version: String,
    is_new_update: bool,
    update_list: Vec<String>,
    all_releases: Vec<Release>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            allow_to_download: true,
            allowed_to_close: true,
            available_version: String::from("0.1.0"),
            update_list: Vec::new(),
            is_new_update: false,
            all_releases: Vec::new(),
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
                    let _ = check_status(&mut self.available_version, &mut self.is_new_update);
                    println!(
                        "[self.available_version] {} [self.is_new_update] {}",
                        self.available_version, self.is_new_update
                    );
                }

                if ui.add(egui::Button::new("Donwload updates")).clicked() {
                    // println!("")
                }

                if ui.add(egui::Button::new("Realeses")).clicked() {
                    let _ = check_rel_list();
                }

                if ui.add(egui::Button::new("Get list all realeses")).clicked() {
                    self.all_releases = get_releases_list().expect("message");
                    println!("{:?}", self.all_releases);
                }

                if ui.add(egui::Button::new("Replace")).clicked() {
                    replace_itself();
                }
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.add(egui::Label::new("List updates:"));

                for release in &self.all_releases {
                    let version = &release.version;
                    // let date = &release.date;

                    // ui.add(egui::Label::new(version.to_owned() + date));
                    ui.add(egui::Label::new(version.to_owned()));
                }

                if ui.add(egui::Button::new("Refresh")).clicked() {
                    self.all_releases = get_releases_list().expect("message");
                    println!("REFRESH: {:?}", self.all_releases);
                }

                ui.separator();
                ui.add(egui::Label::new(APP_VERSION));
            });

            let new_release = "[NEW] Available version: ".to_owned() + &self.available_version;
            if self.is_new_update {
                ui.add(egui::Label::new(new_release));
            }
        });
    }
}
