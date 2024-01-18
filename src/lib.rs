//! `shogi-img` is a library for generating images that visualize the position in Shogi (Japanese chess).
//! It takes [`shogi_core::PartialPosition`] as input and returns an image corresponding to that position as [`image::RgbaImage`].
//! There are several selectable styles for the board and pieces used in image generation.
mod generator;

pub use crate::generator::Generator;
pub use image;
pub use shogi_core;

/// Values to specify the style of the board.
#[derive(Default)]
pub enum BoardStyle {
    #[default]
    /// Light-colored wood
    Light,
    /// Warm-colored wood
    Warm,
    /// Synthetic resin
    Resin,
}

/// Values to specify the style of the pieces.
#[derive(Default)]
pub enum PiecesStyle {
    #[default]
    Hitomoji,
    HitomojiGothic,
}

/// A simple function to generate an image from a position using the default styles.
pub fn pos2img(position: &shogi_core::PartialPosition) -> image::RgbaImage {
    Generator::default().generate(position)
}
