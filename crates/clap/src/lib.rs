//! Mixin a clap argument for colored output selection
//!
//! ## Examples
//!
//! ```rust
//! // ...
//! #[derive(Debug, structopt::StructOpt)]
//! #[structopt(setting = concolor_clap::color_choice())]
//! struct Cli {
//!     #[structopt(flatten)]
//!     color: concolor_clap::Color,
//! }
//! ```

pub use concolor_control::ColorChoice;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, structopt::StructOpt)]
pub struct Color {
    /// Controls when to use color.
    #[structopt(long, default_value = "auto", value_name = "WHEN")]
    color: ColorChoice,
}

impl Color {
    /// Set the user selection on `concolor_control`
    pub fn apply(&self) {
        concolor_control::set(self.when());
    }

    /// Get the user's selection
    pub fn when(&self) -> ColorChoice {
        self.color
    }
}

pub fn color_choice() -> structopt::clap::AppSettings {
    let color = concolor_control::get(concolor_control::Stream::Either);
    if color.ansi_color() {
        structopt::clap::AppSettings::ColorAlways
    } else {
        structopt::clap::AppSettings::ColorNever
    }
}
