use crossbeam::scope;
use crossbeam_channel::{bounded, unbounded};
use dirs::home_dir;
use image::ImageBuffer;
use lazy_static::lazy_static;
use num::complex::Complex;
use num_cpus;
use ring::digest::Digest;
use shared::{compute_digest, is_iso, julia, wavelength_to_rgb};
use std::{
    error::Error,
    io::{Error as IoError, ErrorKind},
    path::PathBuf,
    sync::{
        mpsc::{channel, Receiver},
        Mutex,
    },
    thread,
    time::Duration,
};
use threadpool::ThreadPool;
use walkdir::WalkDir;

/// Find the maximum value in an array
///
/// # Arguments
///
/// * `arr` - A slice of i32 values
///
/// * `threshold` - A usize value that holds the threshold for the array length
///
/// # Returns
///
/// * An Option<i32> which holds the maximum value in the array
///
/// # Examples
///
/// ```
/// use threads::find_max;
///
/// let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let threshold = 5;
/// assert_eq!(Some(10), find_max(&arr, threshold));
/// ```
pub fn find_max(arr: &[i32], threshold: usize) -> Option<i32> {
    if arr.len() <= threshold {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);

    scope(|s| {
        let thread_l = s.spawn(|_| find_max(left, threshold));
        let thread_r = s.spawn(|_| find_max(right, threshold));

        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;

        Some(max_l.max(max_r))
    })
    .unwrap()
}

/// Parallel pipeline, where a source sends messages to a worker, which processes them and sends them to a sink.
///
/// # Arguments
///
/// * `num_messages` - A usize value that holds the number of messages to be sent by the source
/// * `num_workers` - A usize value that holds the number of workers
///
/// # Examples
///
/// ```
/// use threads::parallel_pipeline;
/// parallel_pipeline(10, 2);
/// ```
pub fn parallel_pipeline(num_messages: usize, num_workers: usize) {
    let (sender1, receiver1) = bounded(1);
    let (sender2, receiver2) = bounded(1);

    scope(|s| {
        s.spawn(|_| {
            for i in 0..num_messages {
                sender1.send(i).unwrap();
                println!("Source sent: {}", i);
            }
            drop(sender1);
        });

        for _ in 0..num_workers {
            let (sender, receiver) = (sender2.clone(), receiver1.clone());
            s.spawn(move |_| {
                thread::sleep(Duration::from_millis(500));
                for message in receiver.iter() {
                    println!("Worker {:?} received {}.", thread::current().id(), message);
                    sender.send(message * 2).unwrap();
                }
            });
        }

        drop(sender2);
        for message in receiver2.iter() {
            println!("Sink received: {}", message);
        }
    })
    .unwrap();
}

/// Pass data between two threads
///
/// # Arguments
///
/// * `num_messages` - A usize value that holds the number of messages to be sent
///
/// # Returns
///
/// * A tuple containing the first element as an empty tuple and the second element as a crossbeam_channel::Receiver<usize>
///
/// # Examples
///
/// ```
/// use threads::pass_data_between_two_threads;
/// let (_, receiver) = pass_data_between_two_threads(10);
/// ```
pub fn pass_data_between_two_threads(
    num_messages: usize,
) -> ((), crossbeam_channel::Receiver<usize>) {
    let (sender, receiver) = unbounded();

    (
        scope(|s| {
            s.spawn(|_| {
                for i in 0..num_messages {
                    sender.send(i).unwrap();
                    thread::sleep(Duration::from_millis(100));
                }
            });
        })
        .unwrap(),
        receiver,
    )
}

/// Create a global state
///
/// # Examples
///
/// ```
/// use threads::create_global_state;
///
/// let global_state = create_global_state();
///
/// {
///     let global_state = global_state.lock().unwrap();
///     assert_eq!(global_state.len(), 0);
/// }
/// ```
pub fn create_global_state() -> &'static Mutex<Vec<String>> {
    lazy_static! {
        static ref GLOBAL_STATE: Mutex<Vec<String>> = Mutex::new(Vec::new());
    }

    &GLOBAL_STATE
}

/// Insert a token into the global state
///
/// # Arguments
///
/// * `token` - A string slice that holds the token to be inserted
/// * `state` - A reference to a Mutex<Vec<String>> that holds the global state
///
/// # Returns
///
/// * A Result<(), Box<dyn Error>> where the error is a string slice
///
/// # Examples
///
/// ```
/// use threads::{global_state_insert, create_global_state};
///
/// let global_state = create_global_state();
/// global_state_insert("BTC", global_state).unwrap();
/// global_state_insert("ETH", global_state).unwrap();
/// {
///    let global_state = global_state.lock().unwrap();
///    assert_eq!(global_state.len(), 2);
/// }
pub fn global_state_insert(
    token: &str,
    state: &'static Mutex<Vec<String>>,
) -> Result<(), Box<dyn Error>> {
    let mut global_state = state.lock().map_err(|_| "Failed to acquire MutexGuard")?;
    global_state.push(token.to_string());
    Ok(())
}

/// Calculate the SHA256 sum of ISO files
///
/// # Returns
///
/// * A Result<crossbeam_channel::Receiver<Result<(Digest, PathBuf), IoError>>, IoError> which holds the receiver
///
/// # Examples
///
/// ```
/// use threads::calculate_sha256_sum_of_iso_files;
/// calculate_sha256_sum_of_iso_files().unwrap();
/// ```
pub fn calculate_sha256_sum_of_iso_files(
) -> Result<Receiver<Result<(Digest, PathBuf), IoError>>, IoError> {
    let pool = ThreadPool::new(num_cpus::get());

    let (tx, rx) = channel();

    let home_dir = home_dir().ok_or(IoError::new(
        ErrorKind::NotFound,
        "Home directory not found",
    ))?;
    let download_dir = home_dir.join("Downloads");

    for entry in WalkDir::new(download_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir() && is_iso(e.path()))
    {
        let path = entry.path().to_owned();
        let tx = tx.clone();
        pool.execute(move || {
            let digest = compute_digest(path);
            tx.send(digest).expect("Could not send data!");
        });
    }

    drop(tx);
    Ok(rx)
}

/// Draw a fractal image
///
/// # Arguments
///
/// * `width` - A u32 value that holds the width of the image
/// * `height` - A u32 value that holds the height of the image
/// * `iterations` - A u32 value that holds the number of iterations
/// * `output_file` - A string slice that holds the name of the output file
///
/// # Returns
///
/// * A Result<(), Box<dyn Error>> where the error is a string slice
///
/// # Examples
///
/// ``` ignore
/// use threads::draw_fractal;
/// let output_file = "fractal.png";
/// let result = draw_fractal(1024, 1024, 300, output_file);
/// assert!(result.is_ok(), "Failed to draw fractal: {:?}", result.err());
/// ```
pub fn draw_fractal(
    width: u32,
    height: u32,
    iterations: u32,
    output_file: &str,
) -> Result<(), Box<dyn Error>> {
    let mut img = ImageBuffer::new(width, height);

    let c = Complex::new(-0.8, 0.156);

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    for y in 0..height {
        let tx = tx.clone();
        pool.execute(move || {
            for x in 0..width {
                let i = julia(c, x, y, width, height, iterations);
                let pixel = wavelength_to_rgb(380 + i * 400 / iterations);
                tx.send((x, y, pixel)).expect("Could not send data!");
            }
        });
    }

    for _ in 0..(width * height) {
        let (x, y, pixel) = rx.recv()?;
        img.put_pixel(x, y, pixel);
    }
    let _ = img.save(output_file)?;
    Ok(())
}
