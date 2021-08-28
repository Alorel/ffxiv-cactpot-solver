use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use super::BoardPosition;

pub const MAX_VALUE: u8 = 9;

/// A board position with user selection
#[derive(Eq, PartialEq, Debug, Default, Copy, Clone)]
pub struct ValuedBoardPosition {
    position: BoardPosition,
    value: u8,
}

impl Display for ValuedBoardPosition {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.value, f)
    }
}

impl ValuedBoardPosition {
    #[inline]
    pub fn from_pos(value: u8, position: BoardPosition) -> Self {
        debug_assert!(
            value <= MAX_VALUE,
            "Value must be 1..{}; received: {}",
            MAX_VALUE,
            value
        );

        Self { value, position }
    }

    #[inline]
    pub fn from_u8(value: u8, col: u8, row: u8) -> Self {
        Self::from_pos(value, BoardPosition::new(col, row))
    }

    /// Nothing selected at the given position
    #[inline]
    pub fn empty(position: BoardPosition) -> Self {
        Self::from_pos(0, position)
    }

    #[inline]
    pub fn position(&self) -> BoardPosition {
        self.position
    }

    #[inline]
    pub fn value(&self) -> u8 {
        self.value
    }
}

#[cfg(test)]
mod test {
    use crate::{BoardPosition, ValuedBoardPosition};

    #[test]
    fn from_pos() {
        for value in 1u8..9 {
            for col in 0u8..3 {
                for row in 0u8..3 {
                    let actual = ValuedBoardPosition::from_pos(value, BoardPosition::new(col, row));
                    let exp = ValuedBoardPosition {
                        value,
                        position: BoardPosition::new(col, row),
                    };

                    assert_eq!(exp, actual, "{} @ {},{}", value, col, row);
                }
            }
        }
    }

    #[test]
    fn from_u8() {
        for value in 1u8..9 {
            for col in 0u8..3 {
                for row in 0u8..3 {
                    let actual = ValuedBoardPosition::from_u8(value, col, row);
                    let exp = ValuedBoardPosition {
                        value,
                        position: BoardPosition::new(col, row),
                    };

                    assert_eq!(exp, actual, "{} @ {},{}", value, col, row);
                }
            }
        }
    }

    #[test]
    fn empty() {
        for pos_idx in 0u8..9 {
            let pos = BoardPosition::from_index(pos_idx);
            let actual = ValuedBoardPosition::empty(pos);
            let exp = ValuedBoardPosition {
                value: 0,
                position: BoardPosition::from_index(pos_idx),
            };

            assert_eq!(exp, actual, "{},{}", pos.col, pos.row);
        }
    }
}
