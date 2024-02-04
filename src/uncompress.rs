use std::fs::File;
use std::error::Error;

#[cfg(feature = "use_tar")]
use flate2::read::GzDecoder;

#[cfg(feature = "use_tar")]
use tar::Archive;

pub mod Uncompresser {

    // #[cfg(feature = "use_tar")]
    #[cfg(feature = "tar")]
    pub fn try_decompress_tar(target: String) -> Result<(), Box<dyn std::error::Error>> {
        let path = target.clone();
        println!("hello world: {}", target);

        let tar_gz = File::open(path)?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        archive.unpack(".")?;

        Ok(())
    }

    // #[cfg(not(feature = "use_tar"))]
    // pub fn try_decompress_tar(_target: String) -> Result<(), Box<dyn std::error::Error>> {
    //     // If "use_tar" feature is not enabled, provide an appropriate alternative or do nothing.
    //     println!("hello withoout: {}", _target);

    //     Ok(())
    // }
}
