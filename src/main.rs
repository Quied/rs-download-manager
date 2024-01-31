use eframe::egui;
use webbrowser;

use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

mod Update;
mod View;

use Update::update::check_status;

use egui::widgets::ProgressBar;
use View::view::{show_buttons, show_sys_info};

#[derive(Default)]
struct MyApp {
    download_target: String,
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
    allow_to_download: bool,
    network_information: bool,

    update_list: Vec<String>,

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
    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            
            
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("Check updates")).clicked() {
                    check_status();
                }           

                if ui.add(egui::Button::new("Donwload updates")).clicked(){
                    // println!("")
                }
            });

            ui.separator();


            ui.vertical(|ui| {
                ui.add(egui::Label::new("List updates:"));
            });
            
        });

        

        if self.show_confirmation_dialog {
            frame.close();
        }
    }
}
