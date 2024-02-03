use self_replace::*;
use self_update::cargo_crate_version;
use std::env::temp_dir;
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
        .repo_owner("jaemk")
        .repo_name("self_update") 
        //  .repo_owner("quied")
        // .repo_name("rs-download-manager")
        .build()?
        .fetch()?;
    println!("found releases:");
    println!("{:#?}\n", releases);

    // get the first available release
    let asset = releases[0]
        .asset_for(&self_update::get_target(), None)
        .unwrap();

    println!("1");

    let tmp_dir = tempfile::Builder::new()
        .prefix("new_dir")
        .tempdir_in(::std::env::current_dir()?)?;
    
    let static_dir = std::path::Path::new("/temp/");

    println!("2");
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
            // Successfully opened a file, but it should be a directory
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
    println!("3");

    println!("[ASSET NAME] {:?}", &asset.name);

    let file_path = String::from("./static_dir/".to_owned() + &asset.name);
    // let file_path = String::from("./static_dir/".to_owned());

    // let metadata = file_path.metadata()?;
    // let mut permissions = metadata.permissions();

    // let mut file = OpenOptions::new()
    // .read(true)  // Set to true if you also need read access
    // .write(true) // Set to true for write access
    // .create(true) // Create the file if it doesn't exist
    // .open(&file_path)?;

    // let tmp_tarball = ::std::fs::File::open(&tmp_tarball_path)?;
    // let tmp_tarball = ::std::fs::File::open(file_path)?;
    let tmp_tarball = ::std::fs::File::create(&file_path)?;

    println!("[tmp_tarball] {:?}", tmp_tarball);
    println!("4");

    self_update::Download::from_url(&asset.download_url)
        .set_header(reqwest::header::ACCEPT, "application/octet-stream".parse()?)
        .download_to(&tmp_tarball)?;

    println!("5");

    let extract_target = std::path::Path::new(&file_path);
    let bin_name = std::path::PathBuf::from("self_update_bin");



    println!("[extract_target]: {:?}", extract_target);
    println!("[bin_name]: {:?}", bin_name);
    println!("6");

    //self_update::Extract::from_source(&tmp_tarball_path)
    self_update::Extract::from_source(&extract_target)
        .archive(self_update::ArchiveKind::Plain(Some(
            self_update::Compression::Gz,
        )))
        .extract_file(&tmp_dir.path(), &bin_name)?;

        println!("7");

    let new_exe = tmp_dir.path().join(bin_name);
    // println!("[new_exe]: {:?}", new_exe);
    self_replace::self_replace(new_exe)?;


    println!("end");
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
