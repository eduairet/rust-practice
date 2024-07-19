use std::time::Instant;

pub fn get_two_code_sections_elapsed_time(callback: fn()) -> String {
    let start = Instant::now();
    callback();
    let duration = start.elapsed();
    format!("Elapsed time: {:?}", duration)
}
