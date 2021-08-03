//! Mixin a clap argument for colored output selection
//!
//! ## Examples
//!
//! ```rust
//! // ...
//! #[derive(Debug, structopt::StructOpt)]
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
    /// Get the user's selection
    pub fn when(&self) -> ColorChoice {
        self.color
    }
}
