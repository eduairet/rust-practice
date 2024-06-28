use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use std::{
    fs::{DirBuilder, File},
    io::{Error, ErrorKind},
    path::PathBuf,
};
use tar::{Archive, Builder};

/// Compress a list of directories into a tar.gz file
///
/// # Arguments
///
/// * `file` - A string slice that holds the path to the tar.gz file
///
/// * `path` - A string slice that holds the path to the directories to be compressed
///
/// * `src_path` - A string slice that holds the path to the source directories
///
/// # Examples
///
/// ```
/// use compression::compress_file;
/// use std::fs::remove_file;
///
/// compress_file("test_compress.tar.gz", "backup/logs", "/var/log").unwrap();
/// remove_file("test_compress.tar.gz").unwrap();
/// ```
pub fn compress_file(file: &str, path: &str, src_path: &str) -> Result<(), Error> {
    let tar_gz = File::create(file)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);
    tar.append_dir_all(path, src_path)?;

    Ok(())
}

/// Decompress a tar.gz file into a directory
///
/// # Arguments
///
/// * `file` - A string slice that holds the path to the tar.gz file
///
/// * `output_dir` - A string slice that holds the path to the output directory
///
/// # Examples
///
/// ```
/// use compression::{compress_file, decompress_file};
/// use std::fs::{remove_dir_all, remove_file};
///
/// compress_file("test_decompress.tar.gz", "backup/logs", "/var/log").unwrap();
/// decompress_file("test_decompress.tar.gz", "output_dir_decompress").unwrap();
/// remove_dir_all("output_dir_decompress").unwrap();
/// remove_file("test_decompress.tar.gz").unwrap();
/// ```
pub fn decompress_file(file: &str, output_dir: &str) -> Result<(), Error> {
    let tar_gz = File::open(file)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    DirBuilder::new().recursive(true).create(output_dir)?;
    archive.unpack(output_dir)?;

    Ok(())
}

/// Decompress a tar.gz file into a directory, removing a prefix from the extracted files
///
/// # Arguments
///
/// * `file` - A string slice that holds the path to the tar.gz file
///
/// * `prefix` - A string slice that holds the prefix to be removed from the extracted files
///
/// * `output_dir` - A string slice that holds the path to the output directory
///
/// # Examples
///
/// ```
/// use compression::{compress_file, decompress_removing_prefix};
/// use std::fs::{remove_dir_all, remove_file};
///
/// compress_file("test_decompress_prefix.tar.gz", "backup/logs", "/var/log").unwrap();
/// decompress_removing_prefix("test_decompress_prefix.tar.gz", "backup/logs", "output_dir_decompress_prefix").unwrap();
/// remove_dir_all("output_dir_decompress_prefix").unwrap();
/// remove_file("test_decompress_prefix.tar.gz").unwrap();
/// ```
pub fn decompress_removing_prefix(file: &str, prefix: &str, output_dir: &str) -> Result<(), Error> {
    let tar_gz = File::open(file)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    DirBuilder::new().recursive(true).create(output_dir)?;

    println!("Extracted the following files:");
    archive
        .entries()?
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf, Error> {
            let path = entry
                .path()?
                .strip_prefix(prefix)
                .map_err(|e| Error::new(ErrorKind::Other, e))?
                .to_owned();
            let out_path = PathBuf::from(output_dir).join(&path);
            entry.unpack(&out_path)?;
            Ok(path)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));

    Ok(())
}
