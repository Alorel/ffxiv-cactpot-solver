use std::fmt::{Display, Formatter, Result as FmtResult};

use singletons::BY_POS_LOOKUP;

use crate::board_position::valued_board_position::singletons::EMPTIES;

use super::BoardPosition;

pub const MAX_VALUE: u8 = 9;

/// A board position with user selection
#[derive(Eq, PartialEq, Debug)]
pub struct ValuedBoardPosition {
    position: &'static BoardPosition,
    value: u8,
}

impl ValuedBoardPosition {
    /// Replaces the position with one of the 'static ones
    pub fn as_static(&self) -> &'static Self {
        Self::from_pos(self.value, self.position)
    }

    pub fn from_pos(value: u8, position: &'static BoardPosition) -> &'static Self {
        if value == 0 || value > MAX_VALUE {
            panic!("Value must be 1..{}; received: {}", MAX_VALUE, value);
        }

        &BY_POS_LOOKUP[(value - 1) as usize][position.index as usize]
    }

    pub fn from_u8(value: u8, col: u8, row: u8) -> &'static Self {
        Self::from_pos(value, BoardPosition::new(col, row))
    }

    /// Nothing selected at the given position
    pub fn empty(position: &'static BoardPosition) -> &'static Self {
        &EMPTIES[position.index as usize]
    }

    pub fn position(&self) -> &'static BoardPosition {
        &self.position
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

impl Display for ValuedBoardPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.value)
    }
}

mod singletons {
    use super::super::super::board_position::singletons;
    use super::ValuedBoardPosition as VBP;

    macro_rules! mkrow {
        ($value: expr) => {
            [
                VBP {
                    position: bp!(0),
                    value: $value,
                },
                VBP {
                    position: bp!(1),
                    value: $value,
                },
                VBP {
                    position: bp!(2),
                    value: $value,
                },
                VBP {
                    position: bp!(3),
                    value: $value,
                },
                VBP {
                    position: bp!(4),
                    value: $value,
                },
                VBP {
                    position: bp!(5),
                    value: $value,
                },
                VBP {
                    position: bp!(6),
                    value: $value,
                },
                VBP {
                    position: bp!(7),
                    value: $value,
                },
                VBP {
                    position: bp!(8),
                    value: $value,
                },
            ]
        };
    }

    macro_rules! bp {
        ($idx: expr) => {
            &singletons::LOOKUP_BY_IDX[$idx]
        };
    }

    pub const EMPTIES: [VBP; 9] = [
        VBP {
            position: bp!(0),
            value: 0,
        },
        VBP {
            position: bp!(1),
            value: 0,
        },
        VBP {
            position: bp!(2),
            value: 0,
        },
        VBP {
            position: bp!(3),
            value: 0,
        },
        VBP {
            position: bp!(4),
            value: 0,
        },
        VBP {
            position: bp!(5),
            value: 0,
        },
        VBP {
            position: bp!(6),
            value: 0,
        },
        VBP {
            position: bp!(7),
            value: 0,
        },
        VBP {
            position: bp!(8),
            value: 0,
        },
    ];

    pub const BY_POS_LOOKUP: [[VBP; 9]; 9] = [
        mkrow!(1),
        mkrow!(2),
        mkrow!(3),
        mkrow!(4),
        mkrow!(5),
        mkrow!(6),
        mkrow!(7),
        mkrow!(8),
        mkrow!(9),
    ];
}

#[cfg(test)]
mod test {
    use crate::{BoardPosition, ValuedBoardPosition};

    #[test]
    fn as_static() {
        let src = ValuedBoardPosition {
            position: BoardPosition::default(),
            value: 5,
        };
        let st = ValuedBoardPosition::as_static(&src);

        assert_eq!(&src, st);
    }

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

                    assert_eq!(&exp, actual, "{} @ {},{}", value, col, row);
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

                    assert_eq!(&exp, actual, "{} @ {},{}", value, col, row);
                }
            }
        }
    }

    #[test]
    fn empty() {
        for pos_idx in 0u8..9 {
            let pos = BoardPosition::from_index(pos_idx);
            let actual = ValuedBoardPosition::empty(&pos);
            let exp = ValuedBoardPosition {
                value: 0,
                position: BoardPosition::from_index(pos_idx),
            };

            assert_eq!(&exp, actual, "{},{}", pos.col, pos.row);
        }
    }
}
