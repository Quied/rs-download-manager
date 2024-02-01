//mod Downloader;
//use Downloader::Download::{download_file};

pub mod view {

    pub fn soome(ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| if ui.button("sdf").clicked() {});
    }

    pub fn show_buttons(ctx: &egui::Context, mut organize: bool) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {}); // UI
        });
    }
}
