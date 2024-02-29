pub mod update {
    use std::fs::{set_permissions, File, OpenOptions};
    use std::path::Path;
    use std::env;
    
    use std::{
        fs,
        io::{stdin, Read},
    };

    use self_replace::*;
    use serde_json::Value;
    use std::env::consts::EXE_EXTENSION;
    use std::process::{exit, Command};
    use std::sync::mpsc::channel;
    use std::thread;

    pub enum UpdateStatus {
        ShowUpdateDialog,
        HideUpdateDialog,
        UpdatesChecked,
        UpdatesNotChecked,
    }

    pub enum Msg2MainThread {
        DrawWindow(bool),
    }

    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub struct Sender<T> {
    //     inner: mpmc::Sender<T>,
    // }

    pub struct SharedUpdate {
        show_modal: bool,
    }

    // pub enum ServerResponseStatus { }

    pub fn check_updates() -> Result<(), Box<dyn (::std::error::Error)>> {
        Ok(())
    }

    pub fn self_delete() -> Result<(), Box<dyn std::error::Error>> {
        self_replace::self_delete()?;

        Ok(())
    }

    pub fn replace_itself() -> Result<(), Box<dyn std::error::Error>> {
        let exe = std::env::current_exe().unwrap();
        let new_executable = std::fs::read_link(exe.clone())
            .unwrap_or(exe)
            .with_file_name("Achernar")
            .with_extension(EXE_EXTENSION);

        if !new_executable.is_file() {
            eprintln!("hello does not exist, run cargo build --example hello first.");
            std::process::exit(1);
        }

        println!("Next time I run, I am the hello executable");
        self_replace::self_replace(&new_executable).unwrap();

        if std::env::var("FORCE_EXIT").ok().as_deref() == Some("1") {
            std::process::exit(0);
        }

        Ok(())
    }

    pub fn draw_download_window(status: &mut UpdateStatus, ctx: &egui::Context) {
        egui::Window::new("New version available!")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.add(egui::Label::new("Download it?"));

                    ui.horizontal(|ui| {
                        if ui.button("No").clicked() {
                            println!("close window");
                            *status = UpdateStatus::HideUpdateDialog;
                        }

                        if ui.button("Yes").clicked() {
                            println!("Downloading");
                            download_updates();
                            *status = UpdateStatus::HideUpdateDialog;
                        }
                    })
                });
            });
    }

    pub fn check_for_updates(
        status: &mut UpdateStatus,
        current_version: &String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match status {
            UpdateStatus::UpdatesNotChecked => {
                let target_server = "http://localhost:3002/update".to_owned();
                let client = reqwest::blocking::Client::new();
                let response = client.get(&target_server).send()?;

                if response.status().is_success() {
                    let json_body: Value = response.json()?;
                    let new_app_version = json_body["new_app_version"].as_str();

                    if let Some(version) = new_app_version {
                        dbg!("New update found: {:?}", new_app_version);
                        *status = UpdateStatus::ShowUpdateDialog;
                    } else {
                        dbg!("Out of update");
                        *status = UpdateStatus::UpdatesChecked;
                    }
                } else {
                    println!("Request was not successful: {}", response.status());
                    // *status = UpdateStatus::ServerRequestError;
                    *status = UpdateStatus::UpdatesChecked;
                }

                Ok(())
            }

            UpdateStatus::ShowUpdateDialog => {
                // draw_download_window(status, ctx);
                // send2backend();
                Ok(())
            }

            _ => {
                // other logic
                Ok(())
            }
        }

        //Ok(false)
    }

    pub fn download_updates() -> Result<(), Box<dyn std::error::Error>> {
        let target_point = "http://localhost:3002/download".to_owned();
        let archive_save_path = std::path::Path::new("./downloaded_archive.tar.gz");

        let client = reqwest::blocking::Client::new();
        let mut response = client.get(&target_point).send()?;

        if response.status().is_success() {
            let mut temp_archive = std::fs::File::create(&archive_save_path)?;
            response.copy_to(&mut temp_archive)?;

            extract_archive(&archive_save_path)?;
        } else {
            dbg!("request error{}");
        }

        Ok(())
    }

    pub fn extract_archive(target: &Path) -> Result<(), Box<dyn std::error::Error>> {
        use flate2::read::GzDecoder;
        use tar::Archive;

        let exe = std::env::current_exe().unwrap();
        // let new_exe = std::fs::read_link(exe.clone())
        //     .unwrap_or(exe)
        //     .with_file_name("target")
        //     .with_extension(EXE_EXTENSION);

        let new_exe = "./extracted/hydraulics_app".to_owned();

            // if !new_exe.is_file(){
            //     println!("replace error it's not a file");
            // }
            // else {
            //     self_replace::self_replace(&new_exe).unwrap();
            // }

        let target_exe_name = "hydraulics_app".to_owned();

        let path = target.clone();
        println!("[extracted target] {:?}", target);

        let tar_gz = File::open(path)?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);

        // Delete before archive extract otherwise archive isn't unpack
        

        let extract_archive_path = "./extracted".to_owned();
        archive.unpack(&extract_archive_path)?;

        // self_replace::self_replace("/extracted/hydraulics_app")?;
        self_replace::self_replace(new_exe.clone())?;

        // delete archive
        std::fs::remove_file(target)?;

        // let exe_path = target_exe_name.clone();
        // let dir_path = ".";
        // let _ = Command::new(exe_path)
        //     .current_dir(dir_path)
        //     .spawn()
        //     .expect("Failed to execute command");

        println!("[success extract] {:?}", target);

        exit(0);
        Ok(())
    }
}
