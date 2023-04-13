//! Global override of color control

#![cfg_attr(not(test), no_std)]

pub use colorchoice::ColorChoice;

/// Get the current [`ColorChoice`] state
pub fn get() -> ColorChoice {
    ColorChoice::global()
}

/// Override the detected [`ColorChoice`]
pub fn set(choice: ColorChoice) {
    choice.write_global()
}
