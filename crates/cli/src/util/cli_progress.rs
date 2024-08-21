use colored::Colorize;
use derive_more::Display;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use log::LevelFilter;
use num_format::{Locale, ToFormattedString};
use std::fmt::Write;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub trait CliProgressTrait {
    fn with_type(ty: CliProgressType) -> Self;
    fn increase(&self, current: usize, total: usize);
    fn set_message(&self, msg: &'static str);
    fn finish(&self, ok: bool);
}

pub struct CliProgress {
    pb: Option<ProgressBar>,
    ty: CliProgressType,
}

#[derive(Display)]
pub enum CliProgressType {
    #[display(fmt = "Generat")]
    Generate,
    #[display(fmt = "Serializ")]
    Serialize,
}

impl CliProgress {
    pub fn new(ty: CliProgressType) -> Self {
        Self { pb: None, ty }
    }

    pub fn increase(&mut self, current: usize, total: usize) {
        if self.pb.is_none() {
            if LevelFilter::Info <= log::max_level() {
                log::info!(
                    "{}ing {} records",
                    self.ty,
                    format!("~{}", total.to_formatted_string(&Locale::en))
                );
            } else {
                println!(
                    "{}ing {} records",
                    self.ty,
                    format!("~{}", total.to_formatted_string(&Locale::en)).bright_cyan()
                );
            }

            let len = total.to_string().len() + 1;
            let pb = ProgressBar::new(total as _);
            pb.enable_steady_tick(Duration::from_millis(80));
            pb.set_style(
                ProgressStyle::with_template(
                    &format!("{{spinner:.green}} {{msg}} [{{elapsed_precise}}] |{{wide_bar:.cyan/blue}}| {{cur:>{len}}}/{{total:{len}}} ({{per_sec}})"),
                )
                    .unwrap()
                    .with_key("cur", |state: &ProgressState, w: &mut dyn Write| write!(w, "{}", state.pos().to_formatted_string(&Locale::en)).unwrap())
                    .with_key("total", |state: &ProgressState, w: &mut dyn Write| write!(w, "{}", state.len().unwrap_or_default().to_formatted_string(&Locale::en)).unwrap())
                    .with_key("per_sec", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}/s", state.per_sec()).unwrap())
                    .progress_chars("#>-")
                    .tick_strings(&[
                        "⠋",
                        "⠙",
                        "⠹",
                        "⠸",
                        "⠼",
                        "⠴",
                        "⠦",
                        "⠧",
                        "⠇",
                        "⠏",
                        "✔"
                    ]),
            );
            pb.set_message(format!("{}ing records", self.ty));

            self.pb.replace(pb);
        }

        self.pb.as_ref().unwrap().set_position(current as _);
    }

    pub fn set_message(&self, msg: &'static str) {
        if let Some(pb) = self.pb.as_ref() {
            pb.set_message(msg);
        }
    }

    pub fn finish(&self, ok: bool) {
        if let Some(pb) = self.pb.as_ref() {
            if ok {
                pb.finish_with_message("Done");
                if LevelFilter::Info <= log::max_level() {
                    log::info!(
                        "Successfully {}ed {} records in {:.1?}",
                        self.ty.to_string().to_ascii_lowercase(),
                        pb.position().to_formatted_string(&Locale::en),
                        pb.elapsed()
                    );
                } else {
                    println!(
                        "Success - {}ed {} records in {}",
                        self.ty,
                        pb.position().to_formatted_string(&Locale::en).bright_blue(),
                        format!("{:.1?}", pb.elapsed()).bright_blue()
                    );
                }
            } else {
                pb.abandon_with_message("Error");
                log::error!(
                    "Failed to {}e data",
                    self.ty.to_string().to_ascii_lowercase()
                );
            }
        }
    }
}

pub type CliProgressRef = Arc<Mutex<CliProgress>>;

impl CliProgressTrait for CliProgressRef {
    fn with_type(ty: CliProgressType) -> Self {
        Arc::new(Mutex::new(CliProgress::new(ty)))
    }

    fn increase(&self, current: usize, total: usize) {
        self.lock().unwrap().increase(current, total);
    }

    fn set_message(&self, msg: &'static str) {
        self.lock().unwrap().set_message(msg);
    }

    fn finish(&self, ok: bool) {
        self.lock().unwrap().finish(ok);
    }
}
