pub mod update {
    use self_update::cargo_crate_version;
    use self_update::self_replace;
    use std::io::{stdin, Read};
    use std::{fs, io};

    use self_update::{Download, Extract};
    use std::fs::{set_permissions, File, OpenOptions};
    use std::path::Path;

    use std::env::consts::EXE_EXTENSION;

    pub fn check_updates() -> Result<(), Box<dyn (::std::error::Error)>> {

       Ok(())
    }



    pub fn replace_itself() -> Result<(), Box<dyn std::error::Error>> {
        
        let exe = std::env::current_exe().unwrap();
        let new_executable = std::fs::read_link(exe.clone())
            .unwrap_or(exe)
            .with_file_name("hello")
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
