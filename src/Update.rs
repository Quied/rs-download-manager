// use self_update::cargo_crate_version;


pub mod update {

    use egui::accesskit::kurbo::Arc;
    use reqwest::header::STRICT_TRANSPORT_SECURITY;
    // use crate::Update::cargo_crate_version;
    use self_update::cargo_crate_version;
    use self_update::self_replace;
    use self_update::update::Release;
    use self_update::update::ReleaseUpdate;
    use std::env::consts::EXE_EXTENSION;
    use std::env::current_exe;
    // let bin_name = std::path::PathBuf::from("this_temp_name");

    pub fn check_rel_list() -> Result<(), Box<dyn (::std::error::Error)>> {
        let mut builds = self_update::backends::github::ReleaseList::configure();
        println!("checking on realeses...");

        #[cfg(feature = "signatures")]
        builds.repo_owner("jaemk");

        let releases = builds.repo_name("self_update").build()?.fetch()?;
        //        let releases = builds.repo_name("self_updates").build()?.fetch()?;
        println!("releases: ");
        println!("{:#?}\n", releases);

        Ok(())
    }

    pub fn replace_itself() {
        let APP_NAME = "Achernar".to_owned();

        let exe = std::env::current_exe().unwrap();
        let new_executable = std::fs::read_link(exe.clone())
            .unwrap_or(exe)
            .with_file_name(APP_NAME.clone())
            .with_extension(EXE_EXTENSION);

        if !new_executable.is_file() {
            eprintln!("hello does not exist, run cargo build --example hello first.");
            std::process::exit(1);
        }

        println!("Next time I run, I am the {} executable", APP_NAME);
        self_replace::self_replace(&new_executable).unwrap();

        if std::env::var("FORCE_EXIT").ok().as_deref() == Some("1") {
            std::process::exit(0);
        }
    }

    pub fn check_status(
        new_version: &mut String,
        is_new_upd: &mut bool,
    ) -> Result<(), Box<dyn (::std::error::Error)>> {
        let status = self_update::backends::github::Update::configure()
            .repo_owner("quied")
            .repo_name("rs-download-manager")
            .bin_name("github")
            .no_confirm(true)
            .show_download_progress(true)
            .current_version(cargo_crate_version!())
            .build()?
            .update()?;
        println!("Update status: `{}`!", status.version());

        *new_version = String::from(status.version());
        *is_new_upd = true;

        Ok(())
    }

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
    pub fn get_release_this() -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::prelude::*;

        extern crate tar;

        let releases = self_update::backends::github::ReleaseList::configure()
            .repo_owner("quied")
            .repo_name("rs-download-manager")
            .build()?
            .fetch()?;

        println!("found releases:");
        println!("{:#?}\n", releases);

        let asset = releases[0].asset_for("achernar", None).unwrap();

        let mut test = String::from("./test-folder/");
        test.push_str(&asset.name);

        let bin_name_with_path = File::create(&test);

        // self_update::Download::from_url(&asset.download_url)
        //     .set_header(reqwest::header::ACCEPT, "application/octet-stream".parse()?)
        //     .download_to(bin_name_with_path)?;

        // let bin_name = std::path::PathBuf::from("self_update_bin");
        // self_update::Extract::from_source(&tmp_tarball_path)
        //     .archive(self_update::ArchiveKind::Tar(Some(self_update::Compression::Gz)))
        //     .extract_file(&tmp_dir.path(), &bin_name)?;

        Ok(())
    }

    pub fn test_upd() -> Result<(), Box<dyn (::std::error::Error)>> {
        //     let status = self_update::backends::github::Update::configure()
        //         .repo_owner("andrew121410")
        //         .repo_name("Limonium")
        //         .bin_name("limonium")
        //         .target("limonium")
        //         .no_confirm(true)
        //         .show_download_progress(true)
        //         .show_output(true)
        //         .current_version(cargo_crate_version!())
        //         .build()?;

        //    println!("Update status: `{}`!", status.current_version());
        //     Ok(())

        // use std::io::prelude::*;
        // use tar::Archive;

        //         let releases = self_update::backends::github::ReleaseList::configure()
        //         .repo_owner("jaemk")
        //         .repo_name("self_update")
        //         .build()?
        //         .fetch()?;
        //     println!("found releases:");
        //     println!("{:#?}\n", releases);

        //     // get the first available release
        //     let asset = releases[0]
        //         .asset_for(&self_update::get_target(), None).unwrap();

        //     let tmp_dir = tempfile::Builder::new()
        //             .prefix("self_update")
        //             .tempdir_in(::std::env::current_dir()?)?;
        //     let tmp_tarball_path = tmp_dir.path().join(&asset.name);
        //     let tmp_tarball = ::std::fs::File::open(&tmp_tarball_path)?;

        //     self_update::Download::from_url(&asset.download_url)
        //         .set_header(reqwest::header::ACCEPT, "application/octet-stream".parse()?)
        //         .download_to(&tmp_tarball)?;

        //     let bin_name = std::path::PathBuf::from("self_update_bin");
        //     self_update::Extract::from_source(&tmp_tarball_path)
        //         .archive(self_update::ArchiveKind::Tar(Some(self_update::Compression::Gz)))
        //         .extract_file(&tmp_dir.path(), &bin_name)?;

        //     let new_exe = tmp_dir.path().join(bin_name);
        //     self_replace::self_replace(new_exe)?;

        //     Ok(())
        let status = self_update::backends::github::Update::configure()
            .repo_owner("quied")
            .repo_name("rs-download-manager")
            .bin_name("github")
            .show_download_progress(true)
            .current_version(cargo_crate_version!())
            .no_confirm(true)
            .build()?
            .update()?;
        println!("Update status: `{}`!", status.version());
        Ok(())
    }

    pub fn get_releases_list() -> Result<Vec<Release>, Box<dyn (::std::error::Error)>> {
        let releases = self_update::backends::github::ReleaseList::configure()
            .repo_owner("quied")
            .repo_name("rs-download-manager")
            .build()?
            .fetch()?;
        println!("found releases:");
        println!("[skip]");

        Ok(releases)
    }

    pub fn update_current() -> Result<(), Box<dyn (::std::error::Error)>> {
        let releases = self_update::backends::github::ReleaseList::configure()
            .repo_owner("quied")
            .repo_name("rs-download-manager")
            .build()?
            .fetch()?;
        println!("found releases:");
        println!("{:#?}\n", releases);

        let temp = Some("hell");

        // get the first available release
        let asset = releases[0]
            .asset_for(&self_update::get_target(), temp)
            .unwrap();

        // self_update::Download::from_url(url)

        let tmp_dir = tempfile::Builder::new()
            .prefix("self_update")
            .tempdir_in(::std::env::current_dir()?)?;
        let tmp_tarball_path = tmp_dir.path().join(&asset.name);
        let tmp_tarball = ::std::fs::File::open(&tmp_tarball_path)?;

        self_update::Download::from_url(&asset.download_url)
            .set_header(reqwest::header::ACCEPT, "application/octet-stream".parse()?)
            .download_to(&tmp_tarball)?;

        let bin_name = std::path::PathBuf::from("self_update_bin");
        self_update::Extract::from_source(&tmp_tarball_path)
            // .archive(self_update::ArchiveKind::Tar(Some(self_update::Compression::Gz)))
            .extract_file(&tmp_dir.path(), &bin_name)?;

        let new_exe = tmp_dir.path().join(bin_name);
        self_replace::self_replace(new_exe)?;

        Ok(())
    }
}
