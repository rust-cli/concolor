/// Selection for when to color output
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

#[cfg(feature = "std")]
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

#[cfg(not(feature = "std"))]
impl core::str::FromStr for ColorChoice {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            AUTO => Ok(Self::Auto),
            ALWAYS => Ok(Self::Always),
            NEVER => Ok(Self::Never),
            _ => Err("Unknown color choice"),
        }
    }
}

const AUTO: &str = "auto";
const ALWAYS: &str = "always";
const NEVER: &str = "never";
