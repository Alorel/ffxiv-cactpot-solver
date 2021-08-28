use fill_failure::FillFailure;

use super::end_row::EndRow;
use super::{AvailableSelectionIter, BoardPosition, ValuedBoardPosition};
use smallvec::SmallVec;

pub mod available_selection_iter;
pub mod fill_failure;

const BOARD_CAPACITY: u8 = 9;

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct Board {
    fills: SmallVec<[ValuedBoardPosition; 9]>,
}

impl Board {
    #[inline]
    pub fn clear_fills(&mut self) {
        self.fills.clear();
    }

    #[inline]
    pub fn len(&self) -> u8 {
        self.fills.len() as u8
    }

    #[inline]
    pub fn remaining_capacity(&self) -> u8 {
        BOARD_CAPACITY - self.len()
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.len() == BOARD_CAPACITY
    }

    #[inline]
    pub fn available_selections(&self) -> AvailableSelectionIter {
        AvailableSelectionIter::new(self)
    }

    pub fn col(&self, idx: u8) -> EndRow {
        EndRow::new(
            self.compute_board_pos(idx, 0),
            self.compute_board_pos(idx, 1),
            self.compute_board_pos(idx, 2),
        )
    }

    pub fn row(&self, idx: u8) -> EndRow {
        EndRow::new(
            self.compute_board_pos(0, idx),
            self.compute_board_pos(1, idx),
            self.compute_board_pos(2, idx),
        )
    }

    pub fn diag_tl_br(&self) -> EndRow {
        EndRow::new(
            self.compute_board_pos(0, 0),
            self.compute_board_pos(1, 1),
            self.compute_board_pos(2, 2),
        )
    }

    pub fn diag_bl_tr(&self) -> EndRow {
        EndRow::new(
            self.compute_board_pos(0, 2),
            self.compute_board_pos(1, 1),
            self.compute_board_pos(2, 0),
        )
    }

    pub fn contains_value(&self, v: u8) -> bool {
        self.fills.iter().find(|p| p.value() == v).is_some()
    }

    pub fn contains_position(&self, pos: BoardPosition) -> bool {
        self.fills.iter().find(|p| p.position() == pos).is_some()
    }

    fn compute_board_pos(&self, col: u8, row: u8) -> ValuedBoardPosition {
        self.find_board_pos(col, row)
            .unwrap_or_else(|| ValuedBoardPosition::empty(BoardPosition::new(col, row)))
    }

    #[inline]
    pub fn find(&self, pos: BoardPosition) -> Option<ValuedBoardPosition> {
        self.find_board_pos(pos.col(), pos.row())
    }

    fn find_board_pos(&self, col: u8, row: u8) -> Option<ValuedBoardPosition> {
        self.fills.iter().find_map(|p| {
            if p.position().eq(col, row) {
                Some(p.to_owned())
            } else {
                None
            }
        })
    }

    pub fn fill(&mut self, pos: ValuedBoardPosition) -> Result<(), FillFailure> {
        if self.contains_value(pos.value()) {
            return Err(FillFailure::ValueAlreadyContained);
        } else if self.contains_position(pos.position()) {
            return Err(FillFailure::PositionAlreadyFilled);
        }

        self.fills.push(pos);

        Ok(())
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            fills: SmallVec::with_capacity(BOARD_CAPACITY as usize),
        }
    }
}
