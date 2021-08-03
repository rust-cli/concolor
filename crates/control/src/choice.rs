/// Selection for when to color output
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ColorChoice {
    Auto,
    Always,
    Never,
}

impl Default for ColorChoice {
    fn default() -> Self {
        Self::Auto
    }
}
