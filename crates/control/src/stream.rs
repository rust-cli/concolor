#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Stream {
    Stdout,
    Stderr,
    Either,
}

