use std::fs::{File, Metadata};
use std::io::Read;
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

/// Posts a file to paste.rs and returns the response text.
///
/// # Arguments
///
/// * `message` - A string slice that holds the name of the file to post.
///
/// # Returns
///
/// A Url that holds the response text.
///
/// # Example
///
/// ```ignore
/// use web_programming::clients::downloads::post_file_to_paste_rs;
///
/// #[tokio::main]
/// async fn main() {
///    let message = "message.txt";
///    let response_text = post_file_to_paste_rs(message).await.unwrap();
///    assert!(response_text.contains("https://paste.rs"));
/// }
pub async fn post_file_to_paste_rs(message: &str) -> Result<String, Box<dyn Error>> {
    let paste_api = "https://paste.rs";
    let mut file = File::open(message).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let client = reqwest::Client::new();
    let res = client.post(paste_api).body(contents).send().await.unwrap();
    let response_text = res.text().await.unwrap();
    Ok(response_text)
}
