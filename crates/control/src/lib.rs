//! Control console coloring across all dependencies
//!
//! # `[[bin]]`s
//!
//! ```toml
//! [dependencies]
//! concolor-control = { version = "0.1.0", features = "color" }
//! ```
//!
//! If you are providing a command line option for controlling color, just call
//! ```rust
//! let when = concolor_control::ColorChoice::Always;
//! concolor_control::set(when);
//! ```
//!
//! See also [`concolor-clap`](lib.rs/concolor-clap)
//!
//! # `[lib]`s
//!
//! The `[[bin]]` is responsible for defining the policy of how colors are determined, so to depend
//! on `concolor-control`:
//! ```toml
//! [dependencies]
//! concolor-control = { version = "0.1.0", default-features = false }
//! ```
//!
//! At times, you might want to provide a convenience feature for color support, so you could also:
//! ```toml
//! [features]
//! color = "concolor-control/color"
//!
//! [dependencies]
//! concolor-control = { version = "0.1.0", optional = True}
//! ```
//!
//! Then just ask as needed:
//! ```rust
//! let stdout_support = concolor_control::get(concolor_control::Stream::Stdout);
//! if stdout_support.ansi_color() {
//!     // Output ANSI escape sequences
//!     if stdout_support.truecolor() {
//!         // Get even fancier with the colors
//!     }
//! } else if stdout_support.color() {
//!     // Legacy Windows version, control the console as needed
//! } else {
//!     // No coloring
//! }
//! ```
//!
//! # Features
//!
//! - `color`: Guess color status based on all possible sources, including:
//! - `api`: Allow controlling color via the API
//! - `interactive`: Check if stdout/stderr is a TTY
//! - `clicolor`: Respect [CLICOLOR] spec
//! - `no_color`: Respect [NO_COLOR] spec
//! - `term`: Check `TERM`
//! - `windows`: Check if we can enable ANSI support
//!
//! [CLICOLOR]: https://bixense.com/clicolors/
//! [NO_COLOR]: https://no-color.org/

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "api")]
mod color;
#[cfg(feature = "api")]
pub use color::*;

#[cfg(not(feature = "api"))]
mod no_color;
#[cfg(not(feature = "api"))]
pub use no_color::*;

mod choice;
pub use choice::*;
mod stream;
pub use stream::*;
