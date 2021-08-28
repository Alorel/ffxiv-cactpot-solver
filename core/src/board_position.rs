use std::fmt::{Display, Formatter, Result as FmtResult};

use singletons::*;

use super::end_row::DiagRow;

pub mod valued_board_position;

pub const MAX_IDX: u8 = 8;
pub const MAX_POS: u8 = 2;

/// A position on the board
#[derive(Eq, Debug, Copy, Clone, Default)]
pub struct BoardPosition {
    col: u8,
    row: u8,
    index: u8,
}

impl PartialEq for BoardPosition {
    #[inline]
    fn eq(&self, other: &BoardPosition) -> bool {
        self.index == other.index
    }
}

impl Display for BoardPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.col, f)?;
        f.write_str(", ")?;
        Display::fmt(&self.row, f)
    }
}

impl BoardPosition {
    #[inline]
    pub fn never() -> Self {
        Self {
            col: u8::MAX,
            row: u8::MAX,
            index: u8::MAX,
        }
    }

    #[inline]
    pub fn new(col: u8, row: u8) -> Self {
        debug_assert!(
            col <= MAX_POS && row <= MAX_POS,
            "Invalid col+row {}, {}",
            col,
            row
        );

        Self {
            col,
            row,
            index: Self::pos_index(col, row),
        }
    }

    #[inline]
    fn pos_index(col: u8, row: u8) -> u8 {
        col + (row * 3)
    }

    #[inline]
    pub fn diag_row(&self) -> DiagRow {
        LOOKUP_DIAG_ROW[self.col as usize][self.row as usize]
    }

    pub fn from_index(index: u8) -> Self {
        debug_assert!(index <= MAX_IDX, "Index out of bounds: {}", index);

        let col = index % 3;

        Self {
            col,
            row: (index - col) / 3,
            index,
        }
    }

    #[inline]
    pub fn eq(&self, col: u8, row: u8) -> bool {
        col == self.col && row == self.row
    }

    #[inline]
    pub fn col(&self) -> u8 {
        self.col
    }

    #[inline]
    pub fn row(&self) -> u8 {
        self.row
    }

    #[inline]
    pub fn index(&self) -> u8 {
        self.index
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(BoardPosition::default(), BoardPosition::from_index(0))
    }

    #[test]
    fn never() {
        assert_eq!(
            BoardPosition::never(),
            BoardPosition {
                col: u8::MAX,
                row: u8::MAX,
                index: u8::MAX,
            }
        )
    }

    #[test]
    fn new() {
        for col in 0u8..3 {
            for row in 0u8..3 {
                let pos = BoardPosition::new(col, row);

                assert_eq!(pos.col(), col, "Col at {},{}", col, row);
                assert_eq!(pos.row(), row, "Row at {},{}", col, row);
            }
        }
    }

    #[test]
    fn diag_row() {
        let specs = [
            DiagRow::TopLeftBottomRight,
            DiagRow::None,
            DiagRow::BottomLeftTopRight,
            DiagRow::None,
            DiagRow::Both,
            DiagRow::None,
            DiagRow::BottomLeftTopRight,
            DiagRow::None,
            DiagRow::TopLeftBottomRight,
        ];
        for i in 0..specs.len() {
            let pos = BoardPosition::from_index(i as u8);
            assert_eq!(pos.diag_row(), specs[i], "{}, {}", pos.col, pos.row);
        }
    }

    #[test]
    fn eq_board() {
        let a = BoardPosition {
            col: 0,
            row: 0,
            index: 0,
        };
        let b = BoardPosition {
            col: 0,
            row: 0,
            index: 0,
        };
        assert_eq!(a, b);
    }

    #[test]
    fn from_index() {
        let exps = [
            BoardPosition::new(0, 0),
            BoardPosition::new(1, 0),
            BoardPosition::new(2, 0),
            BoardPosition::new(0, 1),
            BoardPosition::new(1, 1),
            BoardPosition::new(2, 1),
            BoardPosition::new(0, 2),
            BoardPosition::new(1, 2),
            BoardPosition::new(2, 2),
        ];

        for i in 0..exps.len() {
            let fi = BoardPosition::from_index(i as u8);
            let e = exps[i];
            assert_eq!(fi, e, "{},{} (idx {})", e.col, e.row, e.index);
        }
    }
}

pub mod singletons {

    use super::DiagRow::{self, *};

    pub const LOOKUP_DIAG_ROW: [[DiagRow; 3]; 3] = [
        [TopLeftBottomRight, None, BottomLeftTopRight],
        [None, Both, None],
        [BottomLeftTopRight, None, TopLeftBottomRight],
    ];
}
