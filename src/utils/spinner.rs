use console::style;
use indicatif::{ProgressBar, ProgressStyle};

pub struct Spinner {
    pb: ProgressBar,
}

impl Spinner {
    pub fn new(message: &str) -> Self {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(std::time::Duration::from_millis(80));

        Self { pb }
    }

    pub fn stop_with_message(&self, message: &str) {
        self.pb.finish_with_message(format!("✓ {}", style(message).green()));
    }

    pub fn stop_with_error(&self, message: &str) {
        self.pb.finish_with_message(format!("✗ {}", style(message).red()));
    }
}