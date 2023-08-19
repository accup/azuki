use indicatif::{ProgressBar, ProgressFinish, ProgressStyle};

pub fn new_bar(initial_size: u64) -> ProgressBar {
    let template =
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes}";

    let style = ProgressStyle::with_template(template)
        .unwrap()
        .progress_chars("#>-");

    ProgressBar::new(initial_size)
        .with_style(style)
        .with_finish(ProgressFinish::AndLeave)
}
