use std::fs::{File, Metadata};
use std::{error::Error, io::copy};
use tempfile::Builder;

/// Downloads a file from a given URL and saves it to a temporary directory.
///
/// # Arguments
///
/// * `dir_name` - A string slice that holds the name of the temporary directory.
/// * `file_url` - A string slice that holds the URL of the file to download.
///
/// # Returns
///
/// A tuple containing the metadata of the downloaded file and the number of bytes copied.
///
/// # Example
///
/// ```ignore
/// use web_programming::clients::downloads::download_file_to_temp_directory;
///
/// #[tokio::main]
/// async fn main() {
///    let dir_name = "temp";
///    let file_url = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
///    let (dest, bytes) = download_file_to_temp_directory(dir_name, file_url)
///       .await
///       .unwrap();
///    assert!(dest.file_type().is_file());
///    assert!(bytes > 0);
/// }
/// ```
pub async fn download_file_to_temp_directory(
    dir_name: &str,
    file_url: &str,
) -> Result<(Metadata, u64), Box<dyn Error>> {
    let tmp_dir = Builder::new().prefix(dir_name).tempdir().unwrap();
    let target = file_url;
    let response = reqwest::get(target).await.unwrap();

    let mut dest: File = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        let fname = tmp_dir.path().join(fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname).unwrap()
    };
    let dest_path = dest.metadata().unwrap();
    let content = response.text().await.unwrap();
    Ok((dest_path, copy(&mut content.as_bytes(), &mut dest).unwrap()))
}
