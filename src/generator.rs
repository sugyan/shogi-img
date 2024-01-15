use crate::{Board, Piece};
use image::{imageops, io, DynamicImage, ImageFormat};
use shogi_core::{Color, PartialPosition, PieceKind, Square};
use std::io::Cursor;

macro_rules! load_image {
    ($name:expr, $filename:expr) => {
        io::Reader::with_format(
            Cursor::new(include_bytes!(concat!(
                "./data/pieces/",
                $name,
                "/",
                $filename
            ))),
            ImageFormat::Png,
        )
        .decode()
        .expect("decoding image should be success")
    };
}

macro_rules! load_pieces {
    ($name:expr) => {
        [
            [
                load_image!($name, "07.png"),
                load_image!($name, "06.png"),
                load_image!($name, "05.png"),
                load_image!($name, "04.png"),
                load_image!($name, "03.png"),
                load_image!($name, "02.png"),
                load_image!($name, "01.png"),
                load_image!($name, "10.png"),
                load_image!($name, "17.png"),
                load_image!($name, "16.png"),
                load_image!($name, "15.png"),
                load_image!($name, "14.png"),
                load_image!($name, "12.png"),
                load_image!($name, "11.png"),
            ],
            [
                load_image!($name, "27.png"),
                load_image!($name, "26.png"),
                load_image!($name, "25.png"),
                load_image!($name, "24.png"),
                load_image!($name, "23.png"),
                load_image!($name, "22.png"),
                load_image!($name, "21.png"),
                load_image!($name, "30.png"),
                load_image!($name, "37.png"),
                load_image!($name, "36.png"),
                load_image!($name, "35.png"),
                load_image!($name, "34.png"),
                load_image!($name, "32.png"),
                load_image!($name, "31.png"),
            ],
        ]
    };
}

pub struct Generator {
    board: DynamicImage,
    pieces: [[DynamicImage; PieceKind::NUM]; Color::NUM],
}

impl Generator {
    pub fn new(board: Board, piece: Piece) -> Self {
        let board = match board {
            Board::Light => io::Reader::with_format(
                Cursor::new(include_bytes!("./data/board/light.png")),
                ImageFormat::Png,
            )
            .decode(),
            Board::Warm => io::Reader::with_format(
                Cursor::new(include_bytes!("./data/board/warm.png")),
                ImageFormat::Png,
            )
            .decode(),
            Board::Resin => io::Reader::with_format(
                Cursor::new(include_bytes!("./data/board/resin.png")),
                ImageFormat::Png,
            )
            .decode(),
        }
        .expect("decoding image should be success");
        let pieces = match piece {
            Piece::Hitomoji => load_pieces!("hitomoji"),
            Piece::HitomojiGothic => load_pieces!("hitomoji_gothic"),
        };
        Self { board, pieces }
    }
    pub fn generate(&self, position: PartialPosition) -> DynamicImage {
        let mut board = self.board.clone();
        for sq in Square::all() {
            if let Some(piece) = position.piece_at(sq) {
                imageops::overlay(
                    &mut board,
                    &self.pieces[piece.color().array_index()][piece.piece_kind().array_index()],
                    9 + (9 - sq.file() as i64) * 57,
                    10 + 62 * (sq.rank() as i64 - 1),
                );
            }
        }
        board
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new(Board::default(), Piece::default())
    }
}
