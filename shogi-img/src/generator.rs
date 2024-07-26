use crate::{BoardStyle, HighlightSquare, PiecesStyle};
use image::{imageops, io};
use image::{ImageFormat, Rgba, RgbaImage};
use imageproc::drawing;
use rusttype::{Font, Scale};
use shogi_core::{Color, Hand, Move, PartialPosition, Piece, PieceKind, Position, Square};
use std::io::Cursor;

const HAND_WIDTH: u32 = 200;
const HAND_HEIGHT: u32 = 300;

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
        .to_rgba8()
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

pub trait AsPosition {
    fn hand_of_a_player(&self, color: Color) -> Hand;
    fn piece_at(&self, square: Square) -> Option<Piece>;
    fn last_move(&self) -> Option<Move>;
}

impl AsPosition for Position {
    fn hand_of_a_player(&self, color: Color) -> Hand {
        self.hand_of_a_player(color)
    }
    fn piece_at(&self, square: Square) -> Option<Piece> {
        self.piece_at(square)
    }
    fn last_move(&self) -> Option<Move> {
        self.last_move()
    }
}

impl AsPosition for PartialPosition {
    fn hand_of_a_player(&self, color: Color) -> Hand {
        self.hand_of_a_player(color)
    }
    fn piece_at(&self, square: Square) -> Option<Piece> {
        self.piece_at(square)
    }
    fn last_move(&self) -> Option<Move> {
        self.last_move()
    }
}

/// Image generator.
///
/// It loads resources such as images of the board and pieces at initialization.
///
/// ```
/// use shogi_img::{BoardStyle, Generator, PiecesStyle};
/// use shogi_core::PartialPosition;
///
/// let gen = Generator::new(BoardStyle::Light, PiecesStyle::Hitomoji);
/// let img = gen.generate(&PartialPosition::default());
/// assert!(img.width() > 0 && img.height() > 0);
/// ```
pub struct Generator {
    board: RgbaImage,
    pieces: [[RgbaImage; PieceKind::NUM]; Color::NUM],
    highlight: Option<RgbaImage>,
    font: Font<'static>,
}

impl Generator {
    /// Creates a new `Generator` with the specified styles.
    pub fn new(
        board_style: BoardStyle,
        pieces_style: PiecesStyle,
        highlight_square: HighlightSquare,
    ) -> Self {
        let board = match board_style {
            BoardStyle::Light => io::Reader::with_format(
                Cursor::new(include_bytes!("./data/board/light.png")),
                ImageFormat::Png,
            )
            .decode(),
            BoardStyle::Warm => io::Reader::with_format(
                Cursor::new(include_bytes!("./data/board/warm.png")),
                ImageFormat::Png,
            )
            .decode(),
            BoardStyle::Resin => io::Reader::with_format(
                Cursor::new(include_bytes!("./data/board/resin.png")),
                ImageFormat::Png,
            )
            .decode(),
        }
        .expect("decoding image should be success")
        .to_rgba8();
        let pieces = match pieces_style {
            PiecesStyle::Hitomoji => load_pieces!("hitomoji"),
            PiecesStyle::HitomojiGothic => load_pieces!("hitomoji_gothic"),
        };
        let font = Font::try_from_bytes(include_bytes!("./data/fonts/MonaspaceNeon-Regular.otf"))
            .expect("font should be loaded");
        let highlight = match highlight_square {
            HighlightSquare::LastMoveTo => Some(RgbaImage::from_pixel(
                55,
                60,
                Rgba::from([255, 64, 64, 127]),
            )),
            _ => None,
        };
        Self {
            board,
            pieces,
            font,
            highlight,
        }
    }
    /// Generates an image from the specified position.
    pub fn generate<T>(&self, position: &T) -> RgbaImage
    where
        T: AsPosition,
    {
        let mut image = RgbaImage::new(self.board.width() + HAND_WIDTH * 2, self.board.height());
        imageops::overlay(
            &mut image,
            &self.generate_board(position),
            HAND_WIDTH.into(),
            0,
        );
        imageops::overlay(
            &mut image,
            &self.generate_hand(&position.hand_of_a_player(Color::Black)),
            (HAND_WIDTH + self.board.width() + 1).into(),
            (self.board.height() - HAND_HEIGHT).into(),
        );
        imageops::overlay(
            &mut image,
            &imageops::rotate180(&self.generate_hand(&position.hand_of_a_player(Color::White))),
            0,
            0,
        );
        image
    }
    fn generate_board<T>(&self, pos: &T) -> RgbaImage
    where
        T: AsPosition,
    {
        let last_moved = pos.last_move().map(|m| m.to());
        let mut board = self.board.clone();
        for sq in Square::all() {
            if let Some(piece) = pos.piece_at(sq) {
                if Some(sq) == last_moved {
                    if let Some(img) = &self.highlight {
                        imageops::overlay(
                            &mut board,
                            img,
                            8 + 57 * (9 - i64::from(sq.file())),
                            8 + 62 * (i64::from(sq.rank()) - 1),
                        );
                    }
                }
                imageops::overlay(
                    &mut board,
                    &self.pieces[piece.color().array_index()][piece.piece_kind().array_index()],
                    9 + 57 * (9 - i64::from(sq.file())),
                    10 + 62 * (i64::from(sq.rank()) - 1),
                );
            }
        }
        board
    }
    fn generate_hand(&self, hand: &Hand) -> RgbaImage {
        let mut ret = RgbaImage::from_pixel(
            HAND_WIDTH - 1,
            HAND_HEIGHT,
            // Rgba::from([222, 184, 135, u8::MAX]), // burlywood
            Rgba::from([178, 147, 108, u8::MAX]),
        );
        let mut index = 0;
        for pk in Hand::all_hand_pieces()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            if let Some(count) = hand.count(pk) {
                if count == 0 {
                    continue;
                }
                let piece = &self.pieces[Color::Black.array_index()][pk.array_index()];
                let x = 20 + (index % 2) * (piece.width() + 30);
                let y = 20 + (index / 2) * (piece.height() + 10);
                imageops::overlay(&mut ret, piece, x.into(), y.into());
                if count > 1 {
                    drawing::draw_text_mut(
                        &mut ret,
                        Rgba::from([0, 0, 0, u8::MAX]),
                        (x + piece.width()) as i32,
                        (y + piece.height() - 24) as i32,
                        Scale::uniform(24.0),
                        &self.font,
                        &count.to_string(),
                    );
                }
                index += 1;
            }
        }
        ret
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new(
            BoardStyle::default(),
            PiecesStyle::default(),
            HighlightSquare::default(),
        )
    }
}
