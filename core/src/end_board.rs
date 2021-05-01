use super::{Board, BoardPosition, ValuedBoardPosition};
use super::end_row::{DiagRow, EndRow};
use super::parsed_board::ParsedBoard;

/// Possible endings for the current board
#[derive(Debug)]
pub struct EndBoard {
    possibilities: Vec<ParsedBoard>,
}

fn get_payout<T: PartialEq<T> + Copy>(
    p: &ParsedBoard,
    row_or_col: T,
    extract: fn(&EndRow) -> Option<T>,
) -> Option<u16> {
    let rowcol_some = Some(row_or_col);

    p.end_rows()
        .iter()
        .find(|v| extract(v) == rowcol_some)
        .map(|v| v.payout_value())
}

impl EndBoard {
    fn get_avg<T: PartialEq<T> + Copy>(
        &self,
        row_or_col: T,
        extract: fn(&EndRow) -> Option<T>,
    ) -> u16 {
        // let mut total = 0u32;
        let mut count = 0u16;

        let total: u32 = self.possibilities
            .iter()
            .map(|p| get_payout(p, row_or_col, extract))
            .filter(|v| v.is_some())
            .map(|v| v.unwrap() as u32)
            .inspect(|_| {
                count += 1;
            })
            .sum();

        (total / (count as u32)) as u16
    }

    pub fn avg_for_col(&self, col: u8) -> u16 {
        self.get_avg(col, |v| v.get_column())
    }

    pub fn avg_for_row(&self, row: u8) -> u16 {
        self.get_avg(row, |v| v.get_row())
    }

    pub fn avg_for_diag_row(&self, row: DiagRow) -> u16 {
        if row == DiagRow::None {
            panic!("Can't get average for {:?}", row);
        }

        let extract = |v: &EndRow| {
            let r = v.diag_row();

            match r {
                DiagRow::None => None,
                _ => Some(r),
            }
        };

        match row {
            DiagRow::BottomLeftTopRight => self.get_avg(DiagRow::BottomLeftTopRight, extract),
            _ => self.get_avg(DiagRow::TopLeftBottomRight, extract),
        }
    }
}

pub trait EndBoardGenerator {
    fn available_endings(&self) -> EndBoard;
}

struct BoardIterator<'p> {
    board: &'p Board,
}

fn factorial(of_num: usize) -> usize {
    let mut out: usize = of_num;
    for i in (2..of_num).rev() {
        out = out * i;
    }

    out
}

impl<'p> BoardIterator<'p> {
    pub fn new(board: &'p Board) -> Self {
        Self { board }
    }

    fn iterate_internal(board: Board, column_idx: u8, coll: &mut Vec<ParsedBoard>) {
        let position = BoardPosition::from_index(column_idx);
        let next_idx = column_idx + 1;
        if board.contains_position(&position) {
            return Self::iterate_internal(board, next_idx, coll);
        }

        for value in board.available_selections() {
            let mut board = board.clone();
            board
                .fill(ValuedBoardPosition::from_pos(value, position))
                .ok();

            match board.is_full() {
                true => coll.push(ParsedBoard::from_board(board)),
                false => Self::iterate_internal(board, next_idx, coll),
            };
        }
    }

    pub fn iterate(&self) -> Vec<ParsedBoard> {
        let mut out = {
            let cap = factorial(self.board.remaining_capacity() as usize);
            Vec::with_capacity(cap)
        };
        Self::iterate_internal(self.board.clone(), 0, &mut out);

        out
    }
}

impl EndBoardGenerator for Board {
    fn available_endings(&self) -> EndBoard {
        EndBoard {
            possibilities: BoardIterator::new(&self).iterate(),
        }
    }
}
