pub mod update {
    use self_update::cargo_crate_version;
    use self_update::self_replace;
    use std::io::{stdin, Read};
    use std::{fs, io};

    use self_update::{Download, Extract};
    use std::fs::{set_permissions, File, OpenOptions};
    use std::path::Path;

    pub fn check_updates() -> Result<(), Box<dyn (::std::error::Error)>> {
        let status = self_update::backends::github::Update::configure()
            .repo_owner("quied")
            .repo_name("rs-download-manager")
            .bin_name("github")
            .show_download_progress(true)
            .current_version(cargo_crate_version!())
            .build()?
            .update()?;

        println!("Update status: `{}`!", status.version());
        Ok(())
    }

    pub fn download_updates() -> Result<(), Box<dyn std::error::Error>> {
        let releases = self_update::backends::github::ReleaseList::configure()
            // .repo_owner("jaemk")
            // .repo_name("self_update")
            .repo_owner("quied")
            .repo_name("rs-download-manager")
            .build()?
            .fetch()?;
        println!("found releases:");
        println!("{:#?}\n", releases);

        let asset = releases[0]
            .asset_for(&self_update::get_target(), None)
            .unwrap();

        let tmp_dir = tempfile::Builder::new()
            .prefix("new_dir")
            .tempdir_in(::std::env::current_dir()?)?;

        let static_dir = std::path::Path::new("/temp/");

        println!("[tmp_dir] {:?}", tmp_dir);

        let tmp_dir_path = tmp_dir.path();

        if fs::metadata(tmp_dir_path).is_ok() {
            println!("Temporary directory exists: {:?}", tmp_dir_path);
        } else {
            println!("Temporary directory does not exist: {:?}", tmp_dir_path);
        }

        let tmp_dir_path = "./static_dir/";

        // Attempt to open the directory
        match File::open(&tmp_dir_path) {
            Ok(file) => {
                println!("Path exists, but it is a file: {:?}", tmp_dir_path);
            }
            Err(_) => {
                // Check if the directory exists
                if fs::metadata(&tmp_dir_path).is_ok() {
                    println!("Directory exists: {:?}", tmp_dir_path);
                } else {
                    println!("Directory does not exist: {:?}", tmp_dir_path);
                }
            }
        }

        let tmp_tarball_path = tmp_dir.path().join(&asset.name);
        //let tmp_tarball_path = dir_name.path().join(&asset.name);

        println!("[tmp_tarball_path] {:?}", tmp_tarball_path);
        println!("[ASSET NAME] {:?}", &asset.name);

        let file_path = String::from("./static_dir/".to_owned() + &asset.name);

        let tmp_tarball = ::std::fs::File::create(&file_path)?;

        println!("[tmp_tarball] {:?}", tmp_tarball);

        self_update::Download::from_url(&asset.download_url)
            .set_header(reqwest::header::ACCEPT, "application/octet-stream".parse()?)
            .download_to(&tmp_tarball)?;

        println!("5");

        let extract_target = std::path::Path::new(&file_path);
        let file_for_save = std::path::Path::new("./static_dir/");
        let bin_name = std::path::PathBuf::from("target_bin_file");

        println!("[extract_target]: {:?}", extract_target);
        println!("[bin_name]: {:?}", bin_name);
        println!("[tmp_dir]: {:?}", tmp_dir.path());
        println!("[file_for_save]: {:?}", file_for_save);

        println!("6");

        //self_update::Extract::from_source(&tmp_tarball_path)
        self_update::Extract::from_source(&extract_target)
            .archive(self_update::ArchiveKind::Plain(Some(
                self_update::Compression::Gz,
            )))
            .extract_file(&file_for_save, &bin_name)?;

        let new_exe = file_for_save.join(bin_name);
        println!("[new_exe]: {:?}", new_exe);
        self_replace::self_replace(new_exe)?;

        try_decompress_tar(extract_target);
        println!("end");
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

    // pub fn check_status(
    //     new_version: &mut String,
    //     is_new_upd: &mut bool,
    // ) -> Result<(), Box<dyn (::std::error::Error)>> {
    //     let status = self_update::backends::github::Update::configure()
    //         .repo_owner("quied")
    //         .repo_name("rs-download-manager")
    //         .bin_name("github")
    //         .no_confirm(true)
    //         .show_download_progress(true)
    //         .current_version(cargo_crate_version!())
    //         .build()?
    //         .update()?;
    //     println!("Update status: `{}`!", status.version());

    //     *new_version = String::from(status.version());
    //     *is_new_upd = true;

    //     Ok(())
    // }

    // pub fn get_updates() -> Result<(), Box<dyn (::std::error::Error)>> {

    //     let status = self_update::backends::github::Update::configure()
    //         .repo_owner("quied")
    //         .repo_name("rs-download-manager")
    //         .bin_name("github")
    //         .show_download_progress(true)
    //         .current_version(cargo_crate_version!())
    //         .build()?
    //         .update()?;
    //     println!("Update status: `{}`!", status.version());

    //     Ok(())
    // }

    // GPT variant
    // pub fn get_release_this() -> Result<(), Box<dyn std::error::Error>> {
    //     use std::fs::File;
    //     use std::io::prelude::*;

    //     extern crate tar;

    //     let releases = self_update::backends::github::ReleaseList::configure()
    //         .repo_owner("quied")
    //         .repo_name("rs-download-manager")
    //         .build()?
    //         .fetch()?;

    //     println!("found releases:");
    //     println!("{:#?}\n", releases);

    //     let asset = releases[0].asset_for("achernar", None).unwrap();

    //     let mut test = String::from("./test-folder/");
    //     test.push_str(&asset.name);

    //     let bin_name_with_path = File::create(&test);

    //     // self_update::Download::from_url(&asset.download_url)
    //     //     .set_header(reqwest::header::ACCEPT, "application/octet-stream".parse()?)
    //     //     .download_to(bin_name_with_path)?;

    //     // let bin_name = std::path::PathBuf::from("self_update_bin");
    //     // self_update::Extract::from_source(&tmp_tarball_path)
    //     //     .archive(self_update::ArchiveKind::Tar(Some(self_update::Compression::Gz)))
    //     //     .extract_file(&tmp_dir.path(), &bin_name)?;

    //     Ok(())
    // }

    // pub fn test_upd() -> Result<(), Box<dyn (::std::error::Error)>> {
    //     //     let status = self_update::backends::github::Update::configure()
    //     //         .repo_owner("andrew121410")
    //     //         .repo_name("Limonium")
    //     //         .bin_name("limonium")
    //     //         .target("limonium")
    //     //         .no_confirm(true)
    //     //         .show_download_progress(true)
    //     //         .show_output(true)
    //     //         .current_version(cargo_crate_version!())
    //     //         .build()?;

    //     //    println!("Update status: `{}`!", status.current_version());
    //     //     Ok(())

    //     // use std::io::prelude::*;
    //     // use tar::Archive;

    //     //         let releases = self_update::backends::github::ReleaseList::configure()
    //     //         .repo_owner("jaemk")
    //     //         .repo_name("self_update")
    //     //         .build()?
    //     //         .fetch()?;
    //     //     println!("found releases:");
    //     //     println!("{:#?}\n", releases);

    //     //     // get the first available release
    //     //     let asset = releases[0]
    //     //         .asset_for(&self_update::get_target(), None).unwrap();

    //     //     let tmp_dir = tempfile::Builder::new()
    //     //             .prefix("self_update")
    //     //             .tempdir_in(::std::env::current_dir()?)?;
    //     //     let tmp_tarball_path = tmp_dir.path().join(&asset.name);
    //     //     let tmp_tarball = ::std::fs::File::open(&tmp_tarball_path)?;

    //     //     self_update::Download::from_url(&asset.download_url)
    //     //         .set_header(reqwest::header::ACCEPT, "application/octet-stream".parse()?)
    //     //         .download_to(&tmp_tarball)?;

    //     //     let bin_name = std::path::PathBuf::from("self_update_bin");
    //     //     self_update::Extract::from_source(&tmp_tarball_path)
    //     //         .archive(self_update::ArchiveKind::Tar(Some(self_update::Compression::Gz)))
    //     //         .extract_file(&tmp_dir.path(), &bin_name)?;

    //     //     let new_exe = tmp_dir.path().join(bin_name);
    //     //     self_replace::self_replace(new_exe)?;

    //     //     Ok(())
    //     let status = self_update::backends::github::Update::configure()
    //         .repo_owner("jaemk")
    //         .repo_name("self_update")
    //         .bin_name("target")
    //         .show_download_progress(true)
    //         .current_version(cargo_crate_version!())
    //         .no_confirm(true)
    //         .build()?
    //         .update()?;
    //     println!("Update status: `{}`!", status.version());

    //     Ok(())
    // }

    // pub fn get_releases_list() -> Result<Vec<Release>, Box<dyn (::std::error::Error)>> {
    //     let releases = self_update::backends::github::ReleaseList::configure()
    //         .repo_owner("quied")
    //         .repo_name("rs-download-manager")
    //         .build()?
    //         .fetch()?;
    //     println!("found releases:");
    //     println!("[skip]");

    //     Ok(releases)
    // }

    // pub fn update_current() -> Result<(), Box<dyn (::std::error::Error)>> {
    //     let releases = self_update::backends::github::ReleaseList::configure()
    //         .repo_owner("quied")
    //         .repo_name("rs-download-manager")
    //         .build()?
    //         .fetch()?;
    //     println!("found releases:");
    //     println!("{:#?}\n", releases);

    //     let temp = Some("hell");

    //     // get the first available release
    //     let asset = releases[0]
    //         .asset_for(&self_update::get_target(), temp)
    //         .unwrap();

    //     // self_update::Download::from_url(url)

    //     let tmp_dir = tempfile::Builder::new()
    //         .prefix("self_update")
    //         .tempdir_in(::std::env::current_dir()?)?;
    //     let tmp_tarball_path = tmp_dir.path().join(&asset.name);
    //     let tmp_tarball = ::std::fs::File::open(&tmp_tarball_path)?;

    //     self_update::Download::from_url(&asset.download_url)
    //         .set_header(reqwest::header::ACCEPT, "application/octet-stream".parse()?)
    //         .download_to(&tmp_tarball)?;

    //     let bin_name = std::path::PathBuf::from("self_update_bin");
    //     self_update::Extract::from_source(&tmp_tarball_path)
    //         // .archive(self_update::ArchiveKind::Tar(Some(self_update::Compression::Gz)))
    //         .extract_file(&tmp_dir.path(), &bin_name)?;

    //     let new_exe = tmp_dir.path().join(bin_name);
    //     self_replace::self_replace(new_exe)?;

    //     Ok(())
    // }
}
