//! Mixin a clap argument for colored output selection
//!
//! ## Examples
//!
//! ```rust
//! // ...
//! #[derive(Debug, clap::Parser)]
//! #[clap(color = concolor_clap::color_choice())]
//! struct Cli {
//!     #[command(flatten)]
//!     color: concolor_clap::Color,
//! }
//! ```
//!
//! ## Features
//!
//! - `auto` (default): Automatically detect color support

/// Get color choice for initializing the `clap::App`
pub fn color_choice() -> clap::ColorChoice {
    let color = concolor::get(concolor::Stream::Either);
    if color.ansi_color() {
        clap::ColorChoice::Always
    } else {
        clap::ColorChoice::Never
    }
}

/// Mixin a clap argument for colored output selection
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, clap::Args)]
pub struct Color {
    /// Controls when to use color.
    #[arg(long, default_value_t = ColorChoice::Auto, value_name = "WHEN", value_enum, global = true)]
    pub color: ColorChoice,
}

impl Color {
    /// Set the user selection on `concolor`
    #[cfg(feature = "api_unstable")]
    pub fn apply(&self) {
        concolor::set(self.to_control());
    }

    /// Get the user's selection
    pub fn to_control(&self) -> concolor::ColorChoice {
        match self.color {
            ColorChoice::Auto => concolor::ColorChoice::Auto,
            ColorChoice::Always => concolor::ColorChoice::Always,
            ColorChoice::Never => concolor::ColorChoice::Never,
        }
    }
}

/// Argument value for when to color output
#[derive(Copy, Clone, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum ColorChoice {
    Auto,
    Always,
    Never,
}

impl ColorChoice {
    /// Report all `possible_values`
    pub fn possible_values() -> impl Iterator<Item = clap::builder::PossibleValue> {
        use clap::ValueEnum;
        Self::value_variants()
            .iter()
            .filter_map(clap::ValueEnum::to_possible_value)
    }
}

impl Default for ColorChoice {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for ColorChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use clap::ValueEnum;
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_app() {
        #[derive(Debug, clap::Parser)]
        struct Cli {
            #[clap(flatten)]
            color: Color,
        }

        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
