mod generator;

pub use crate::generator::Generator;
pub use image;
pub use shogi_core;

#[derive(Default)]
pub enum Board {
    #[default]
    Light,
    Warm,
    Resin,
}

#[derive(Default)]
pub enum Piece {
    #[default]
    Hitomoji,
    HitomojiGothic,
}
