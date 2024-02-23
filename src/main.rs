use eframe::egui;

mod Update;
mod thread_update;

use thread_update::thread_update::Updater;

use Update::update::{
    check_for_updates, check_updates, download_updates, replace_itself, self_delete, UpdateStatus,
};

pub enum Msg2BackendThread {
    DownloadUpdate(bool),
}

// #[derive(Default)]
struct MyApp {
    update_status: UpdateStatus,
    is_new_update: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        // let mut backend_thread = Updater::default();
        // backend_thread.thread_update_main();

        Self {
            update_status: UpdateStatus::UpdatesNotChecked,
            is_new_update: false,
        }
    }
}

pub fn draw_download_window(is_new_update: &mut bool, ctx: &egui::Context) {
    egui::Window::new("New version available!")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add(egui::Label::new("Download it?"));

                ui.horizontal(|ui| {
                    if ui.button("No").clicked() {
                        println!("close window");

                        *is_new_update = false;
                        // send to back end thr
                        // ...
                    }

                    if ui.button("Yes").clicked() {
                        println!("Downloading");
                        download_updates();

                        *is_new_update = false;
                        // send to back end thr
                        // ...
                    }
                })
            });
        });
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
                ui.add(egui::Label::new("0.0.0"));
                ui.separator();

                if ui.add(egui::Button::new("Check updates")).clicked() {
                    dbg!("Main thread");
                }

                if ui.add(egui::Button::new("Replace itself")).clicked() {
                    // let _ = replace_itself();
                }

                if ui.add(egui::Button::new("Self delete")).clicked() {
                    // let _ = self_delete();
                }

                if ui.add(egui::Button::new("Server Reqwest")).clicked() {
                    let _ = download_updates();
                }
            });

            draw_download_window(&mut self.is_new_update, ctx);

            ui.separator();

            ui.vertical(|ui| {
                ui.add(egui::Label::new("List updates:"));
            });
        });
    }
}
