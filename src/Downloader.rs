
pub mod Download {
    
    use std::path::Path;
    use std::fs::File;
    use std::io::prelude::*;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::io::Read;
    use std::error::Error;

    extern crate reqwest;
    extern crate tempfile;

    use std::io::copy;
    use tempfile::Builder;
    use reqwest::Client;

    use std::io::Cursor;
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
 

    pub async fn download_file(url: String, file_name: String) -> Result<()> {
        let response = reqwest::get(url).await?;
        let mut file = std::fs::File::create(file_name)?;
        let mut content =  Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut file)?;
        Ok(())
    }
}