use reqwest::{
    blocking::Client,
    header::{CONTENT_LENGTH, RANGE},
    StatusCode,
};
use shared::PartialRangeIter;
use std::{
    error::Error,
    fs::{File, Metadata},
    io::{copy, Read},
    str::FromStr,
};
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

/// Downloads a file in chunks and saves it to the current directory.
///
/// # Arguments
///
/// * `chunk_size` - A u32 that holds the size of the chunks to download.
/// * `duration` - A u32 that holds the duration of the download.
///
/// # Returns
///
/// A File that holds the downloaded file.
///
/// # Example
///
/// ```ignore
/// use web_programming::clients::downloads::make_partial_download;
///
/// let chunk_size = 10240;
/// let duration = 2;
/// let file = make_partial_download(chunk_size, duration).unwrap();
/// assert!(file.metadata().unwrap().len() > 0);
/// ```
pub fn make_partial_download(chunk_size: u32, duration: u32) -> Result<File, Box<dyn Error>> {
    let url: String = format!(
        "https://httpbin.org/range/{}?duration={}",
        chunk_size, duration
    );

    let client = Client::new();
    let response = client.head(&url).send().unwrap();
    let length = response
        .headers()
        .get(CONTENT_LENGTH)
        .ok_or("response doesn't include the content length")
        .unwrap();
    let length = u64::from_str(length.to_str().unwrap())
        .map_err(|_| "invalid Content-Length header")
        .unwrap();

    let mut output_file = File::create("download.bin").unwrap();

    println!("starting download...");
    for range in PartialRangeIter::new(0, length - 1, chunk_size / 10).unwrap() {
        println!("range {:?}", range);
        let mut response = client.get(&url).header(RANGE, range).send().unwrap();

        let status = response.status();
        if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
            panic!("unexpected status code: {:?}", status);
        }
        copy(&mut response, &mut output_file).unwrap();
    }

    let content = response.text().unwrap();
    copy(&mut content.as_bytes(), &mut output_file).unwrap();

    Ok(output_file)
}
