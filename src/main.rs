use self_replace::*;
use self_update::cargo_crate_version;
use std::error;
use std::io::{stdin, Read};
use std::{fs, io};

use self_update::backends::github::ReleaseList;
use self_update::{Download, Extract};
use std::fs::{set_permissions, File, OpenOptions};
use std::path::Path;
use std::thread::Builder;
use tempfile::tempdir_in;

// use tar::Archive;

// #[cfg(feature = "archive-tar")]
// use tar::Builder;

// #[cfg(feature = "archive-tar")]
fn update_for_new() -> Result<(), Box<dyn std::error::Error>> {

    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("quied")
        .repo_name("rs-download-manager")
        .build()?
        .fetch()?;
    println!("found releases:");
    println!("{:#?}\n", releases);

    // get the first available release
    let asset = releases[0].asset_for(&self_update::get_target(), None).unwrap();

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
        .archive(self_update::ArchiveKind::Plain(Some(
            self_update::Compression::Gz,
        )))
        .extract_file(&tmp_dir.path(), &bin_name)?;

    let new_exe = tmp_dir.path().join(bin_name);
    self_replace::self_replace(new_exe)?;

    Ok(())
}

fn update() -> Result<(), Box<dyn (::std::error::Error)>> {
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

fn update_this() -> Result<(), Box<dyn (std::error::Error)>> {
    let mut target: String = String::new();
    println!("1] self_update");

    stdin().read_line(&mut target).expect("msg");
    let x = target.trim().parse().expect("not integer");

    println!("{}", target);

    match x {
        1 => {
            println!("this input is 1");
            update();
        }

        2 => {
            println!("this input is 2");
            update_for_new();
        }

        _ => {
            println!("none");
        }
    }

    Ok(())
}

fn main() {
    println!("Hello, world!");

    let _ = update_this();

}
