use std::fmt::{Display, Formatter, Result as FmtResult};

use singletons::*;

use super::end_row::DiagRow;

pub mod valued_board_position;

pub const MAX_IDX: u8 = 8;
pub const MAX_POS: u8 = 2;

/// A position on the board
#[derive(Eq, Debug)]
pub struct BoardPosition {
    col: u8,
    row: u8,
    index: u8,
}

impl PartialEq<BoardPosition> for BoardPosition {
    fn eq(&self, other: &BoardPosition) -> bool {
        self.index == other.index
    }
}

impl BoardPosition {
    pub fn default() -> &'static Self {
        &P0
    }

    pub fn never() -> &'static Self {
        &NEVER
    }

    pub fn new(col: u8, row: u8) -> &'static Self {
        if col > MAX_POS || row > MAX_POS {
            panic!("Invalid col+row: {}, {}", col, row);
        }

        &LOOKUP_BY_POS[col as usize][row as usize]
    }

    pub fn diag_row(&self) -> &'static DiagRow {
        &LOOKUP_DIAG_ROW[self.col as usize][self.row as usize]
    }

    pub fn from_index(idx: u8) -> &'static Self {
        if idx > MAX_IDX {
            panic!("Index out of bounds: {}", idx);
        }

        &LOOKUP_BY_IDX[idx as usize]
    }

    pub fn eq(&self, col: u8, row: u8) -> bool {
        col == self.col && row == self.row
    }

    pub fn col(&self) -> u8 {
        self.col
    }

    pub fn row(&self) -> u8 {
        self.row
    }

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
            &BoardPosition {
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
            &DiagRow::TopLeftBottomRight,
            &DiagRow::None,
            &DiagRow::BottomLeftTopRight,
            &DiagRow::None,
            &DiagRow::Both,
            &DiagRow::None,
            &DiagRow::BottomLeftTopRight,
            &DiagRow::None,
            &DiagRow::TopLeftBottomRight,
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
    use super::BoardPosition as BP;
    use super::DiagRow::{self, *};

    pub const LOOKUP_BY_POS: [[&'static BP; 3]; 3] =
        [[&P0, &P3, &P6], [&P1, &P4, &P7], [&P2, &P5, &P8]];

    pub const LOOKUP_BY_IDX: [&'static BP; 9] = [&P0, &P1, &P2, &P3, &P4, &P5, &P6, &P7, &P8];

    pub const LOOKUP_DIAG_ROW: [[DiagRow; 3]; 3] = [
        [TopLeftBottomRight, None, BottomLeftTopRight],
        [None, Both, None],
        [BottomLeftTopRight, None, TopLeftBottomRight],
    ];

    pub const P0: BP = BP {
        col: 0,
        row: 0,
        index: 0,
    };
    pub const P1: BP = BP {
        col: 1,
        row: 0,
        index: 1,
    };
    pub const P2: BP = BP {
        col: 2,
        row: 0,
        index: 2,
    };
    pub const P3: BP = BP {
        col: 0,
        row: 1,
        index: 3,
    };
    pub const P4: BP = BP {
        col: 1,
        row: 1,
        index: 4,
    };
    pub const P5: BP = BP {
        col: 2,
        row: 1,
        index: 5,
    };
    pub const P6: BP = BP {
        col: 0,
        row: 2,
        index: 6,
    };
    pub const P7: BP = BP {
        col: 1,
        row: 2,
        index: 7,
    };
    pub const P8: BP = BP {
        col: 2,
        row: 2,
        index: 8,
    };
    pub const NEVER: BP = BP {
        col: u8::MAX,
        row: u8::MAX,
        index: u8::MAX,
    };
}

impl Display for BoardPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}, {}", self.col, self.row)
    }
}
