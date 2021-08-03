mod choice;
mod lazy;

use choice::AtomicChoice;

#[derive(Clone, Debug)]
pub struct Color {
    flags: InternalFlags,
    choice: crate::ColorChoice,
    stream: crate::Stream,
}

impl Color {
    pub fn truecolor(&self) -> bool {
        self.ansi_color() && self.flags.contains(InternalFlags::TRUECOLOR)
    }

    pub fn color(&self) -> bool {
        match self.choice {
            crate::ColorChoice::Auto => self.flags.color(self.stream),
            crate::ColorChoice::Always => true,
            crate::ColorChoice::Never => false,
        }
    }

    pub fn ansi_color(&self) -> bool {
        match self.choice {
            crate::ColorChoice::Auto => self.flags.ansi_color(self.stream),
            crate::ColorChoice::Always => true,
            crate::ColorChoice::Never => false,
        }
    }
}

static FLAGS: lazy::Lazy = lazy::Lazy::new();
static USER: AtomicChoice = AtomicChoice::new();

pub fn get(stream: crate::Stream) -> Color {
    let flags = FLAGS.get_or_init(init);
    Color {
        flags: InternalFlags::from_bits(flags).unwrap(),
        choice: USER.get(),
        stream,
    }
}

pub fn set(choice: crate::ColorChoice) {
    USER.set(choice)
}

fn init() -> usize {
    let mut flags = InternalFlags::empty();

    if cfg!(feature = "clicolor") {
        if concolor_query::clicolor() {
            flags |= InternalFlags::CLICOLOR;
        }
        if concolor_query::clicolor_force() {
            flags |= InternalFlags::CLICOLOR_FORCE;
        }
    } else {
        // Spec defaults to enabled
        flags |= InternalFlags::CLICOLOR;
    }
    if cfg!(feature = "no_color") && concolor_query::no_color() {
        flags |= InternalFlags::NO_COLOR;
    }
    if cfg!(feature = "term") {
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
    if cfg!(feature = "interactive") {
        if atty::is(atty::Stream::Stdout) {
            flags |= InternalFlags::TTY_STDOUT;
        }
        if atty::is(atty::Stream::Stderr) {
            flags |= InternalFlags::TTY_STDERR;
        }
    }
    if cfg!(feature = "windows") && concolor_query::windows::enable_ansi_colors().unwrap_or(false) {
        flags |= InternalFlags::WIN_ANSI;
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
        const TRUECOLOR      = 0b00000100000;
        const WIN_ANSI       = 0b00001000000;
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
            && (self.contains(Self::ANSI_SUPPORT) || self.contains(Self::WIN_ANSI))
            && self.contains(Self::CLICOLOR)
            && !self.contains(Self::NO_COLOR))
            || self.contains(Self::CLICOLOR_FORCE)
    }

    fn is_interactive(self, stream: crate::Stream) -> bool {
        self.intersects(stream.flags())
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
