pub mod update {
    use egui::Response;
    use std::fs::{set_permissions, File, OpenOptions};
    use std::path::Path;
    use std::{
        fs,
        io::{stdin, Read},
    };
    // use reqwest::
    use self_replace::*;
    use std::env::consts::EXE_EXTENSION;

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

    pub fn server_reqwest(url: String) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let mut responce = client.get(&url).send()?;


        if responce.status().is_success() {
            let mut new_file = std::fs::File::create("downloaded_archive.tar.gz")?;

            responce.copy_to(&mut new_file);
            dbg!("Download...");

            let archive_path = std::path::Path::new("./downloaded_archive.tar.gz");
            try_decompress_tar(archive_path);

        } else {
            dbg!("Reqwest error:  {}", responce.status());
        }

        Ok(())
    }

    pub fn try_decompress_tar(target: &Path) -> Result<(), Box<dyn std::error::Error>> {
        use flate2::read::GzDecoder;
        use tar::Archive;

        let path = target.clone();
        println!("[decompress target] {:?}", target);

        let tar_gz = File::open(path)?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        archive.unpack(".")?;

        println!("[success decompressed] {:?}", target);
        Ok(())
    }
}
