use ring::digest::{Context, Digest, SHA256};
use std::{
    fs::File,
    io::{BufReader, Error, Read},
    path::Path,
};

/// Compute the digest of a file
///
/// # Arguments
///
/// * `filepath` - A path to a file
///
/// # Returns
///
/// * A Result<(Digest, P), Error> which holds the digest of the file and the path to the file
///
/// # Examples
///
/// ```
/// use shared::compute_digest;
///
/// let (digest, filepath) = compute_digest("Cargo.toml").unwrap();
/// println!("{:?}", digest);
/// ```
pub fn compute_digest<P: AsRef<Path>>(filepath: P) -> Result<(Digest, P), Error> {
    let mut buf_reader = BufReader::new(File::open(&filepath)?);
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = buf_reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok((context.finish(), filepath))
}

/// Check if a file is an ISO file
///
/// # Arguments
///
/// * `path` - A reference to a Path
///
/// # Returns
///
/// * A boolean value
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use shared::is_iso;
///
/// let path = Path::new("ubuntu.iso");
/// assert_eq!(true, is_iso(&path));
/// ```
pub fn is_iso(path: &Path) -> bool {
    path.extension().map_or(false, |ext| ext == "iso")
}
