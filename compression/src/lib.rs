use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use std::{
    fs::{DirBuilder, File},
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};
use tar::{Archive, Builder};

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
/// ```no_run
/// use compression::{decompress_file, compress_file};
///
/// decompress_file("decompress.tar.gz", "decompress_output").unwrap();
/// ```
pub fn decompress_file(file: &str, output_dir: &str) -> Result<(), Error> {
    let tar_gz = File::open(file)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    DirBuilder::new().recursive(true).create(output_dir)?;
    archive.unpack(".")?;

    Ok(())
}

/// Compress a list of directories into a tar.gz file
///
/// # Arguments
///
/// * `file` - A string slice that holds the path to the tar.gz file
///
/// * `dirs` - A vector of string slices that holds the paths to the directories to be compressed
///
/// # Examples
///
/// ```no_run
/// use compression::compress_file;
///
/// compress_file("compress.tar.gz", vec!["compress"]).unwrap();
/// ```
pub fn compress_file(file: &str, dirs: Vec<&str>) -> Result<(), Error> {
    let tar_gz = File::create(file)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);
    for dir in dirs {
        if !Path::new(dir).exists() {
            DirBuilder::new().recursive(true).create(dir).unwrap();
        }
        tar.append_dir_all(".", dir)?;
    }

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
/// # Examples
///
/// ```no_run
/// use compression::decompress_removing_prefix;
///
/// decompress_removing_prefix("decompress.tar.gz", "decompress_output").unwrap();
/// ```
pub fn decompress_removing_prefix(file: &str, prefix: &str) -> Result<(), Error> {
    let file = File::open(file)?;
    let mut archive = Archive::new(GzDecoder::new(file));

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
            entry.unpack(&path)?;
            Ok(path)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));

    Ok(())
}
