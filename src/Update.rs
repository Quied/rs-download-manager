// use self_update::cargo_crate_version;

pub mod update {

    // use crate::Update::cargo_crate_version;
    use self_update::cargo_crate_version;
    use self_update::self_replace;
    // let bin_name = std::path::PathBuf::from("this_temp_name");


   pub fn check_status() -> Result<(), Box<dyn(::std::error::Error)>> {
        let status = self_update::backends::github::Update::configure()
            .repo_owner("jaemk")
            .repo_name("self_update")
        .bin_name("github")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
    }
 
   pub fn update() -> Result<(), Box<dyn(::std::error::Error)>> {
        let status = self_update::backends::github::Update::configure()
            .repo_owner("jaemk")
            .repo_name("self_update")
            .bin_name("github")
            .show_download_progress(true)
            .current_version(cargo_crate_version!())
            .build()?
            .update()?;
        println!("Update status: `{}`!", status.version());
        Ok(())
    }

    

/*
pub fn release() -> Result<(), Box<dyn(::std::error::Error)>> {
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("jaemk")
        .repo_name("self_update")
        .build()?
        .fetch()?;
    println!("found releases:");
    println!("{:#?}\n", releases);

    // get the first available release
    let asset = releases[0]
        .asset_for(&self_update::get_target()).unwrap();

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
        .archive(self_update::ArchiveKind::Tar(Some(self_update::Compression::Gz)))
        .extract_file(&tmp_dir.path(), &bin_name)?;

    let new_exe = tmp_dir.path().join(bin_name);
    self_replace::self_replace(new_exe)?;

    Ok(())
}
*/


}
