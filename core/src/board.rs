use fill_failure::FillFailure;

use super::{AvailableSelectionIter, BoardPosition, ValuedBoardPosition};
use super::end_row::EndRow;

pub mod available_selection_iter;
pub mod fill_failure;

const BOARD_CAPACITY: u8 = 9;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Board {
    fills: Vec<&'static ValuedBoardPosition>,
}

impl Board {
    pub fn len(&self) -> u8 {
        self.fills.len() as u8
    }

    pub fn remaining_capacity(&self) -> u8 {
        BOARD_CAPACITY - self.len()
    }

    pub fn is_full(&self) -> bool {
        self.len() == BOARD_CAPACITY
    }

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

    pub fn contains_position(&self, pos: &BoardPosition) -> bool {
        self.fills.iter().find(|p| p.position() == pos).is_some()
    }

    fn compute_board_pos(&self, col: u8, row: u8) -> &'static ValuedBoardPosition {
        self.find_board_pos(col, row)
            .unwrap_or_else(|| ValuedBoardPosition::empty(BoardPosition::new(col, row)))
    }

    fn find_board_pos(&self, col: u8, row: u8) -> Option<&'static ValuedBoardPosition> {
        self.fills
            .iter()
            .find(|p| p.position().eq(col, row))
            .map(|p| p.as_static())
    }

    pub fn fill(&mut self, pos: &'static ValuedBoardPosition) -> Result<(), FillFailure> {
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
            fills: Vec::with_capacity(BOARD_CAPACITY as usize),
        }
    }
}
