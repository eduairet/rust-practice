use image::{imageops::FilterType, ImageError, Rgb};
use num::complex::Complex;
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
/// * A `Result<(Digest, P), Error>` which holds the digest of the file and the path to the file
///
/// # Examples
///
/// ```
/// use shared::compute_digest;
///
/// let (digest, filepath) = compute_digest("Cargo.toml").unwrap();
/// assert_eq!(digest.as_ref().len(), 32);
/// assert_eq!(filepath, "Cargo.toml");
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

/// Maps wavelength to RGB values
///
/// # Arguments
///
/// * `wavelength` - A u32 value
///
/// # Returns
///
/// * An `Rgb<u8>` value
///
/// # Examples
///
/// ```
/// use shared::wavelength_to_rgb;
/// use image::Rgb;
///
/// let wavelength = 500;
/// let rgb = wavelength_to_rgb(wavelength);
/// assert_eq!(rgb, Rgb([0, 255, 146]));
/// ```
pub fn wavelength_to_rgb(wavelength: u32) -> Rgb<u8> {
    let wave = wavelength as f32;

    let (r, g, b) = match wavelength {
        380..=439 => ((440. - wave) / (440. - 380.), 0.0, 1.0),
        440..=489 => (0.0, (wave - 440.) / (490. - 440.), 1.0),
        490..=509 => (0.0, 1.0, (510. - wave) / (510. - 490.)),
        510..=579 => ((wave - 510.) / (580. - 510.), 1.0, 0.0),
        580..=644 => (1.0, (645. - wave) / (645. - 580.), 0.0),
        645..=780 => (1.0, 0.0, 0.0),
        _ => (0.0, 0.0, 0.0),
    };

    let factor = match wavelength {
        380..=419 => 0.3 + 0.7 * (wave - 380.) / (420. - 380.),
        701..=780 => 0.3 + 0.7 * (780. - wave) / (780. - 700.),
        _ => 1.0,
    };

    let (r, g, b) = (
        normalize(r, factor),
        normalize(g, factor),
        normalize(b, factor),
    );
    Rgb([r, g, b])
}

/// Maps Julia set distance estimation to intensity values
///
/// # Arguments
///
/// * `c` - A complex number
/// * `x` - A u32 value
/// * `y` - A u32 value
/// * `width` - A u32 value
/// * `height` - A u32 value
/// * `max_iter` - A u32 value
///
/// # Returns
///
/// * A `u32` value
///
/// # Examples
///
/// ```
/// use shared::julia;
/// use num::complex::Complex;
///
/// let c = Complex::new(-0.8, 0.156);
/// let x = 0;
/// let y = 0;
/// let width = 1024;
/// let height = 1024;
/// let max_iter = 300;
/// let i = julia(c, x, y, width, height, max_iter);
/// assert_eq!(i, 0);
/// ```
pub fn julia(c: Complex<f32>, x: u32, y: u32, width: u32, height: u32, max_iter: u32) -> u32 {
    let width = width as f32;
    let height = height as f32;

    let mut z = Complex {
        // scale and translate the point to image coordinates
        re: 3.0 * (x as f32 - 0.5 * width) / width,
        im: 2.0 * (y as f32 - 0.5 * height) / height,
    };

    let mut i = 0;
    for t in 0..max_iter {
        if z.norm() >= 2.0 {
            break;
        }
        z = z * z + c;
        i = t;
    }
    i
}

/// Normalizes color intensity values within RGB range
///
/// # Arguments
///
/// * `color` - A f32 value
/// * `factor` - A f32 value
///
/// # Returns
///
/// * A `u8` value
///
/// # Examples
///
/// ```
/// use shared::normalize;
///
/// let color = 0.5;
/// let factor = 0.5;
/// let normalized_color = normalize(color, factor);
/// assert_eq!(normalized_color, 84);
/// ```
pub fn normalize(color: f32, factor: f32) -> u8 {
    ((color * factor).powf(0.8) * 255.) as u8
}

/// Create a thumbnail image
///
/// # Arguments
///
/// * `original` - A reference to a Path
/// * `thumb_dir` - A reference to a Path
/// * `longest_edge` - A u32 value
///
/// # Returns
///
/// * A `Result<(), ImageError>`
///
/// # Examples
///
/// ```ignore
/// use shared::make_thumbnail;
/// use std::{path::Path, fs::remove_dir_all};
///
/// let original = Path::new("image.jpg"); // Needs to exist
/// let thumb_dir = Path::new("thumbnails");
/// if !thumb_dir.exists() {
///    std::fs::create_dir(thumb_dir).unwrap();
/// }
/// let longest_edge = 256;
/// make_thumbnail(original, thumb_dir, longest_edge).unwrap();
/// remove_dir_all(thumb_dir).unwrap();
/// ```
pub fn make_thumbnail<PA, PB>(
    original: PA,
    thumb_dir: PB,
    longest_edge: u32,
) -> Result<(), ImageError>
where
    PA: AsRef<Path>,
    PB: AsRef<Path>,
{
    let img = image::open(original.as_ref())?;
    let file_path = thumb_dir.as_ref().join(original);

    Ok(img
        .resize(longest_edge, longest_edge, FilterType::Nearest)
        .save(file_path)?)
}
