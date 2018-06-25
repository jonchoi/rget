use indicatif::{ProgressBar, ProgressStyle};

static PBAR_FMT: &'static str = "{msg} {spinner:.green} {percent}% [{wide_bar:.cyan/blue}] {bytes}/{total_byes} eta: {eta}";

pub fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    let bar = match quiet_mode {
        true => ProgressBar::hidden(),
        false => {
            match length {
                Some(len) => ProgressBar::new(len),
                None => ProgressBar::new_spinner(),
            };
        }
    }

    progbar.set_message(msg);
    match length.is_some() {
        true => progbar.set_style(ProgressStyle::default_bar()
                                  .template(PBAR_FMT)
                                  .progress_chars("=> "));
        false => progbar.set_style(ProgressStyle::default_spinner());
    };

    bar
}

