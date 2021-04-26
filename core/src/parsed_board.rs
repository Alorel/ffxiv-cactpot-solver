use std::cmp::Ordering;

use super::Board;
use super::end_row::EndRow;

pub type Rows = [EndRow; 8];

#[derive(Debug)]
pub struct ParsedBoardPayouts {
    avg: u16,
    max: u16,
    min: u16,
}

#[derive(Debug)]
pub struct ParsedBoard {
    payouts_board: ParsedBoardPayouts,
    payouts_row: [u16; 3],
    payouts_col: [u16; 3],
    payout_bl_tr: u16,
    payout_tl_br: u16,
    end_rows: Rows,
    board: Board,
}

impl ParsedBoard {
    pub fn from_board(board: Board) -> Self {
        let r1 = board.row(0);
        let r2 = board.row(1);
        let r3 = board.row(2);

        let c1 = board.col(0);
        let c2 = board.col(1);
        let c3 = board.col(2);

        let bl_tr = board.diag_bl_tr();
        let tl_br = board.diag_tl_br();

        let payouts_row = [r1.payout_value(), r2.payout_value(), r3.payout_value()];
        let payouts_col = [c1.payout_value(), c2.payout_value(), c3.payout_value()];
        let payout_bl_tr = bl_tr.payout_value();
        let payout_tl_br = tl_br.payout_value();

        let end_rows = [r1, r2, r3, c1, c2, c3, bl_tr, tl_br];

        let payouts_board = {
            let mut min: u16 = u16::MAX;
            let mut max = u16::MIN;
            let avg = {
                let mut total = 0u16;

                for row in end_rows.iter() {
                    let v = row.payout_value();
                    if v < min {
                        min = v;
                    }
                    if v > max {
                        max = v;
                    }
                    total += v;
                }

                total / (end_rows.len() as u16)
            };

            ParsedBoardPayouts { avg, max, min }
        };

        ParsedBoard {
            payouts_board,
            payouts_row,
            payouts_col,
            payout_bl_tr,
            payout_tl_br,
            board,
            end_rows,
        }
    }

    pub fn end_rows(&self) -> &Rows {
        &self.end_rows
    }
}

impl PartialEq<ParsedBoard> for ParsedBoard {
    fn eq(&self, other: &ParsedBoard) -> bool {
        self.end_rows == other.end_rows && self.board == other.board
    }
}

impl Eq for ParsedBoard {}

impl PartialOrd<ParsedBoard> for ParsedBoard {
    fn partial_cmp(&self, other: &ParsedBoard) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ParsedBoard {
    fn cmp(&self, other: &Self) -> Ordering {
        self.payouts_board
            .avg
            .partial_cmp(&other.payouts_board.avg)
            .unwrap_or_else(|| self.payouts_board.max.cmp(&other.payouts_board.max))
    }
}
