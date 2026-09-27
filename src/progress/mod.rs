use indicatif::{ProgressBar, ProgressStyle};

pub fn create_progress(total: usize, message: &str) -> ProgressBar {
    let bar = ProgressBar::new(total as u64);

    bar.set_style(ProgressStyle::with_template("{msg} [{bar:40.cyan/blue}] {pos}/{len}").unwrap());

    bar.set_message(message.to_string());

    bar
}
