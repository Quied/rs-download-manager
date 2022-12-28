// mod Network;

// use crate::measurements::MeasurementWindow;
use webbrowser;
use eframe::egui;

use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

mod View;
// mod Downloader;

//use Downloader::Download::{download_file };
use View::view::{show_sys_info, show_buttons};

use egui::widgets::ProgressBar;

// use Network::Wireless::say;

#[derive(Default)]
struct MyApp {
    download_target : String,
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
    allow_to_download : bool,
    network_information : bool,
}
#[tokio::main]
async fn main() { 
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
            
            let mut sys = sysinfo::System::new_all();
            sys.refresh_all();

    

            // ==========================================
            ui.horizontal(|ui| {
                  ui.label("Name: ");
                  ui.label(sys.host_name().unwrap());
                  ui.label("     OS: ");
                  ui.label(sys.name().unwrap());
                  ui.label("     RAM: ");
                  ui.label(((sys.total_memory() / 1024) / 1024).to_string());
                  ui.label("/");
                  ui.label(((sys.used_memory() / 1024) / 1024).to_string());
                  ui.label("     CPU: ");
                  ui.label(sys.cpus().len().to_string());
                  ui.label("     Network: ");
                  for (interface_name, data) in sys.networks() {
                    ui.label(data.received().to_string());
                }
                ui.label("/");
                for (interface_name, data) in sys.networks() {
                    ui.label(data.transmitted().to_string());
                }
            });


            ui.separator();

       
            
            

            // Button action
            ui.horizontal(|ui| {
                    if ui.button("Download").clicked(){         
                        if self.allow_to_download == true { self.allow_to_download = false;}
                          else { self.allow_to_download = true; }  
                        } 
                    
                    if ui.button("Network Information").clicked(){ /*  https://github.com/andy31415/rs-value-plotter */}
                    if ui.button("Most Large").clicked(){  }
                    if ui.button("Clear history").clicked(){   }
                    if ui.button("Organize").clicked(){
                    //    download_file("https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQLFI1-AhIesT1APn2g7HqMorvF7AJs3EcpTuOx6hN7Sb4-KEJOAaTiK34ol7nYxraupN0&usqp=CAU".to_string(), "ou.jpeg".to_string()).await.unwrap();
                        // Hide not essential infomarion
                    } 
                    if ui.button("Report").clicked(){ if webbrowser::open("https://github.com/Quied/").is_ok() {} }
            });
            
            // Show download line
            if self.allow_to_download == true {
                ui.horizontal(|ui| {
                  ui.text_edit_singleline(&mut self.download_target);
                  if ui.button("Ok").clicked(){
                        ui.separator();

                            // download menu


                        ui.separator();
                    // download_file("/folder/file.n");
                  }
                }); 
            }



          // ==========================================
        });

        

        fn menu_bar(ui: &mut egui::Ui){
            use egui::{menu, Button};

            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        // …
                    }
                });
            });
        }

        if self.show_confirmation_dialog {
            // Show confirmation dialog:
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.show_confirmation_dialog = false;
                        }

                        if ui.button("Yes!").clicked() {
                            self.allowed_to_close = true;
                            frame.close();
                        }
                    });
                });
        }
    }
}
