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

/// Get color choice for initializing the `clap::App`
pub fn color_choice() -> structopt::clap::AppSettings {
    let color = concolor_control::get(concolor_control::Stream::Either);
    if color.ansi_color() {
        structopt::clap::AppSettings::ColorAlways
    } else {
        structopt::clap::AppSettings::ColorNever
    }
}

/// Mixin a clap argument for colored output selection
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, structopt::StructOpt)]
pub struct Color {
    /// Controls when to use color.
    #[structopt(long, default_value = "auto", value_name = "WHEN")]
    pub color: ColorChoice,
}

impl Color {
    /// Set the user selection on `concolor_control`
    pub fn apply(&self) {
        concolor_control::set(self.to_control());
    }

    /// Get the user's selection
    pub fn to_control(&self) -> concolor_control::ColorChoice {
        match self.color {
            ColorChoice::Auto => concolor_control::ColorChoice::Auto,
            ColorChoice::Always => concolor_control::ColorChoice::Always,
            ColorChoice::Never => concolor_control::ColorChoice::Never,
        }
    }
}

/// Argument value for when to color output
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorChoice {
    Auto,
    Always,
    Never,
}

impl ColorChoice {
    /// All color choices
    pub const fn choices() -> &'static [Self] {
        &[ColorChoice::Auto, ColorChoice::Always, ColorChoice::Never]
    }

    /// All color choice argument values
    pub const fn values() -> &'static [&'static str] {
        &[AUTO, ALWAYS, NEVER]
    }

    pub const fn value(self) -> &'static str {
        match self {
            Self::Auto => AUTO,
            Self::Always => ALWAYS,
            Self::Never => NEVER,
        }
    }
}

impl Default for ColorChoice {
    fn default() -> Self {
        Self::Auto
    }
}

impl core::fmt::Display for ColorChoice {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.value().fmt(f)
    }
}

impl std::str::FromStr for ColorChoice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            AUTO => Ok(Self::Auto),
            ALWAYS => Ok(Self::Always),
            NEVER => Ok(Self::Never),
            other => Err(format!("Unknown color choice '{}'", other)),
        }
    }
}

const AUTO: &str = "auto";
const ALWAYS: &str = "always";
const NEVER: &str = "never";
