mod generator;

pub use crate::generator::Generator;

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
