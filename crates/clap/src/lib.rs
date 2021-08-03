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

/// Selection for when to color output
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorChoice {
    Auto,
    Always,
    Never,
}

impl ColorChoice {
    /// All color choices
    pub fn choices() -> &'static [Self] {
        &[ColorChoice::Auto, ColorChoice::Always, ColorChoice::Never]
    }

    /// All color choice argument values
    pub fn values() -> &'static [&'static str] {
        &["auto", "always", "never"]
    }
}

impl Default for ColorChoice {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::str::FromStr for ColorChoice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Self::Auto),
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            other => Err(format!("Unknown color choice '{}'", other)),
        }
    }
}
