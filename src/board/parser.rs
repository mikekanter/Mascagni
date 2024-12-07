use core::fmt;
use std::str::FromStr;
use super::Board;
use crate::types::{Castling, Color, Square};

pub enum FenParseErr {
    /// Fen is missing data
    MissingData,
    InvalidPieceType,
    InvalidColor,
    InvalidEnPassant,
}

impl fmt::Display for FenParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let disp = match self {
            FenParseErr::MissingData => "Missing Data",
            FenParseErr::InvalidColor => "Invalid Color",
            FenParseErr::InvalidPieceType => "Invalid Piece Type",
            FenParseErr::InvalidEnPassant => "Invalid En Passant",
        };
        write!(f, "{}", disp)
    }
}

impl FromStr for Board {
    type Err = FenParseErr;

    /// Parses a FEN string into a Board
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Self::default();
        let mut elements = s.split_whitespace();

        let rows = elements.next().ok_or(FenParseErr::MissingData)?.split('/');

        for (rank, row) in rows.rev().enumerate() {
            let mut file = 0;
            for symbol in row.chars() {
                if let Some(skip) = symbol.to_digit(10) {
                    file += skip as u8;
                    continue;
                }
                let piece = symbol.try_into().map_err(|()| FenParseErr::InvalidPieceType)?;
                let color = if symbol.is_uppercase() { Color::White } else { Color::Black };
                let square = Square::from_rank_file(rank as u8, file);

                file += 1;
                board.add_piece(square, color, piece)
            }
        }

        board.side_to_move = match elements.next() {
            Some("w") => Color::White,
            Some("b") => Color::Black,
            _ => return Err(FenParseErr::InvalidColor),
        };

        // TODO: parse the castling rights
        board.state.castling = Castling::from(elements.next().unwrap());

        board.state.en_passant = elements.next().unwrap().try_into().map_err(|()| FenParseErr::InvalidEnPassant)?;

        board.state.halfmove_clock = elements.next().unwrap().parse::<u8>().unwrap();

        board.state.fullmove_number = elements.next().unwrap().parse::<u16>().unwrap();

        Ok(board)
    }
}
