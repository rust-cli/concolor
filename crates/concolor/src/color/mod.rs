mod lazy;

/// Current color state for a [`Stream`][crate::Stream]
///
/// Note: if you hold onto this around calls to [`set`][crate::set], it will be inaccurate.
#[derive(Clone, Debug)]
pub struct Color {
    flags: InternalFlags,
    choice: crate::ColorChoice,
    stream: crate::Stream,
}

impl Color {
    /// Should color be used?
    ///
    /// Note: if supporting wincon coloring, fallback to ANSI if this returns `true`.
    pub fn color(&self) -> bool {
        match self.choice {
            crate::ColorChoice::Auto => self.flags.color(self.stream),
            crate::ColorChoice::AlwaysAnsi => true,
            crate::ColorChoice::Always => true,
            crate::ColorChoice::Never => false,
        }
    }

    /// Should use ANSI coloring?
    #[cfg(not(windows))]
    pub fn ansi_color(&self) -> bool {
        match self.choice {
            crate::ColorChoice::Auto => self.flags.ansi_color(self.stream),
            crate::ColorChoice::AlwaysAnsi => true,
            crate::ColorChoice::Always => true,
            crate::ColorChoice::Never => false,
        }
    }

    /// Should use ANSI coloring?
    #[cfg(windows)]
    pub fn ansi_color(&self) -> bool {
        match self.choice {
            crate::ColorChoice::Auto => self.flags.ansi_color(self.stream),
            crate::ColorChoice::AlwaysAnsi => true,
            crate::ColorChoice::Always => self.flags.intersects(InternalFlags::ANSI_ANY),
            crate::ColorChoice::Never => false,
        }
    }

    /// Should use ANSI truecolor?
    pub fn truecolor(&self) -> bool {
        self.ansi_color() && self.flags.contains(InternalFlags::TRUECOLOR)
    }
}

static FLAGS: lazy::Lazy = lazy::Lazy::new();

/// Get the current [`Color`] state for a given [`Stream`][crate::Stream]
pub fn get(stream: crate::Stream) -> Color {
    let flags = FLAGS.get_or_init(init);
    #[cfg(feature = "api")]
    let choice = match concolor_override::get() {
        concolor_override::ColorChoice::Auto => crate::ColorChoice::Auto,
        concolor_override::ColorChoice::AlwaysAnsi => crate::ColorChoice::AlwaysAnsi,
        concolor_override::ColorChoice::Always => crate::ColorChoice::Always,
        concolor_override::ColorChoice::Never => crate::ColorChoice::Never,
    };
    #[cfg(not(feature = "api"))]
    let choice = crate::ColorChoice::Auto;
    Color {
        flags: InternalFlags::from_bits(flags).unwrap(),
        choice,
        stream,
    }
}

/// Override the detected [`ColorChoice`][crate::ColorChoice]
#[cfg(feature = "api")]
pub fn set(choice: crate::ColorChoice) {
    let choice = match choice {
        crate::ColorChoice::Auto => concolor_override::ColorChoice::Auto,
        crate::ColorChoice::AlwaysAnsi => concolor_override::ColorChoice::AlwaysAnsi,
        crate::ColorChoice::Always => concolor_override::ColorChoice::Always,
        crate::ColorChoice::Never => concolor_override::ColorChoice::Never,
    };
    concolor_override::set(choice)
}

fn init() -> usize {
    let mut flags = InternalFlags::empty();

    #[cfg(feature = "clicolor")]
    {
        if concolor_query::clicolor().unwrap_or(true) {
            flags |= InternalFlags::CLICOLOR;
        }
        if concolor_query::clicolor_force() {
            flags |= InternalFlags::CLICOLOR_FORCE;
        }
    }
    #[cfg(not(feature = "clicolor"))]
    {
        // Spec defaults to enabled
        flags |= InternalFlags::CLICOLOR;
    }

    #[cfg(feature = "no_color")]
    if concolor_query::no_color() {
        flags |= InternalFlags::NO_COLOR;
    }

    #[cfg(feature = "term")]
    {
        if concolor_query::term_supports_color() {
            flags |= InternalFlags::TERM_SUPPORT;
        }
        if concolor_query::term_supports_ansi_color() {
            flags |= InternalFlags::ANSI_SUPPORT;
        }
        if concolor_query::truecolor() {
            flags |= InternalFlags::TRUECOLOR;
        }
    }
    #[cfg(not(feature = "term"))]
    {
        // Don't block color on lack of `term` support, acting as if this field doesn't exist
        flags |= InternalFlags::TERM_SUPPORT;
        if cfg!(not(windows)) {
            // Limit to non-windows platforms as windows natively support wincon instead of ANSI
            flags |= InternalFlags::ANSI_SUPPORT;
        }
    }

    #[cfg(feature = "interactive")]
    {
        use is_terminal::IsTerminal;
        use std::io::{stderr, stdout};
        if stdout().is_terminal() {
            flags |= InternalFlags::TTY_STDOUT;
        }
        if stderr().is_terminal() {
            flags |= InternalFlags::TTY_STDERR;
        }
    }
    #[cfg(not(feature = "interactive"))]
    {
        // Don't block color on lack of `interactive` support, acting as if these fields doesn't
        // exist
        flags |= InternalFlags::TTY_STDOUT;
        flags |= InternalFlags::TTY_STDERR;
    }

    // No fallback when disabled since something has to enable ANSI support on Windows
    #[cfg(feature = "windows")]
    if concolor_query::windows::enable_ansi_colors().unwrap_or(false) {
        flags |= InternalFlags::ANSI_WIN;
    }

    flags.bits()
}

bitflags::bitflags! {
    struct InternalFlags: usize {
        const CLICOLOR       = 0b00000000001;
        const CLICOLOR_FORCE = 0b00000000010;
        const NO_COLOR       = 0b00000000100;
        const TERM_SUPPORT   = 0b00000001000;
        const ANSI_SUPPORT   = 0b00000010000;
        const ANSI_WIN       = 0b00000100000;
        const ANSI_ANY       = 0b00000110000;
        const TRUECOLOR      = 0b00001000000;
        const TTY_STDOUT     = 0b00010000000;
        const TTY_STDERR     = 0b00100000000;
        const TTY_ANY        = 0b00110000000;
    }
}

impl InternalFlags {
    fn color(self, stream: crate::Stream) -> bool {
        (self.is_interactive(stream)
            && self.contains(Self::TERM_SUPPORT)
            && self.contains(Self::CLICOLOR)
            && !self.contains(Self::NO_COLOR))
            || self.contains(Self::CLICOLOR_FORCE)
    }

    fn ansi_color(self, stream: crate::Stream) -> bool {
        (self.is_interactive(stream)
            && self.contains(Self::TERM_SUPPORT)
            && self.intersects(Self::ANSI_ANY)
            && self.contains(Self::CLICOLOR)
            && !self.contains(Self::NO_COLOR))
            || self.contains(Self::CLICOLOR_FORCE)
    }

    fn is_interactive(self, stream: crate::Stream) -> bool {
        self.contains(stream.flags())
    }
}

impl crate::Stream {
    fn flags(self) -> InternalFlags {
        match self {
            Self::Stdout => InternalFlags::TTY_STDOUT,
            Self::Stderr => InternalFlags::TTY_STDERR,
            Self::Either => InternalFlags::TTY_ANY,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_interactive() {
        let flags = InternalFlags::empty();
        assert!(!flags.is_interactive(crate::Stream::Stdout));
        assert!(!flags.is_interactive(crate::Stream::Stderr));
        assert!(!flags.is_interactive(crate::Stream::Either));

        let flags = InternalFlags::TTY_STDOUT;
        assert!(flags.is_interactive(crate::Stream::Stdout));
        assert!(!flags.is_interactive(crate::Stream::Stderr));
        assert!(!flags.is_interactive(crate::Stream::Either));

        let flags = InternalFlags::TTY_STDERR;
        assert!(!flags.is_interactive(crate::Stream::Stdout));
        assert!(flags.is_interactive(crate::Stream::Stderr));
        assert!(!flags.is_interactive(crate::Stream::Either));

        let flags = InternalFlags::TTY_STDOUT | InternalFlags::TTY_STDERR;
        assert!(flags.is_interactive(crate::Stream::Stdout));
        assert!(flags.is_interactive(crate::Stream::Stderr));
        assert!(flags.is_interactive(crate::Stream::Either));
    }
}
