use super::{Board, BOARD_CAPACITY};

const LIMIT: u8 = BOARD_CAPACITY + 1;

/// Iterate through the board for available tiles to select
pub struct AvailableSelectionIter<'b> {
    board: &'b Board,
    counter: u8,
    num_emitted: usize,
    will_emit: usize,
}

impl<'b> Iterator for AvailableSelectionIter<'b> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_emitted_everything() {
            return None;
        }

        let counter: u8 = self.counter;
        if counter == LIMIT {
            return None;
        }

        self.counter += 1;

        match self.board.contains_value(counter) {
            true => self.next(),
            false => {
                self.num_emitted += 1;
                Some(counter)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let v = self.will_emit - self.num_emitted;

        (v, Some(v))
    }
}

impl<'b> AvailableSelectionIter<'b> {
    /// Returns true if the number of emitted items equals the projected emission count
    fn has_emitted_everything(&self) -> bool {
        self.num_emitted == self.will_emit
    }

    pub fn new(board: &'b Board) -> AvailableSelectionIter<'_> {
        AvailableSelectionIter {
            board,
            counter: 1,
            num_emitted: 0,
            will_emit: BOARD_CAPACITY as usize - board.fills.len(),
        }
    }
}
