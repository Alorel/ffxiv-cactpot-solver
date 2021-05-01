use super::{Board, BoardPosition};
use super::end_board::EndBoardGenerator;
use super::end_row::DiagRow;

#[derive(Debug)]
pub struct Recommendation {
    avg_col: [u16; 3],
    avg_row: [u16; 3],
    avg_tl_br: u16,
    avg_bl_tr: u16,
    max_avg: u16,
    suggestions: Vec<&'static BoardPosition>,
}

fn validate_board(b: &Board) -> Result<(), &'static str> {
    let f = b.len();
    if f == 0 {
        return Err("The first position is chosen for you");
    } else if f > 4 {
        return Err("Time to pick a row");
    }

    Ok(())
}

impl Recommendation {
    fn cmp_num(num: u16, max: u16) -> u8 {
        match num == max {
            true => 1,
            false => 0,
        }
    }

    fn calc_max_avg(averages: &[u16; 8], previous_attempt_average: u16) -> Option<u16> {
        let max_avg = averages.iter();
        let max_avg = match previous_attempt_average == u16::MAX {
            true => max_avg.max(),
            false => max_avg.filter(|v| v < &&previous_attempt_average).max(),
        };

        max_avg.map(|v| *v)
    }

    fn mk_suggestions(
        board: &Board,
        averages: &[u16; 8],
        curr_max_avg: u16,
    ) -> Vec<&'static BoardPosition> {
        let mut suggestions = Vec::with_capacity(1);
        let mut curr_max_matches = 0u8;

        for col in 0..3 {
            // 1 = true, 0 = false
            let matches_col = Self::cmp_num(averages[col], curr_max_avg);

            for row in 0..3 {
                let pos = BoardPosition::new(col as u8, row as u8);
                if board.contains_position(&pos) {
                    continue;
                }

                // 1 = true, 0 = false
                let matches_row = Self::cmp_num(averages[row + 3], curr_max_avg);

                let diag_plus: u8 = match pos.diag_row() {
                    DiagRow::Both => {
                        Self::cmp_num(averages[6], curr_max_avg)
                            + Self::cmp_num(averages[7], curr_max_avg)
                    }
                    DiagRow::None => 0,
                    DiagRow::BottomLeftTopRight => Self::cmp_num(averages[7], curr_max_avg),
                    DiagRow::TopLeftBottomRight => Self::cmp_num(averages[6], curr_max_avg),
                };

                let total_plus = matches_col + matches_row + diag_plus;
                if total_plus == curr_max_matches {
                    suggestions.push(pos);
                } else if total_plus > curr_max_matches {
                    curr_max_matches = total_plus;
                    suggestions.clear();
                    suggestions.push(pos);
                }
            }
        }

        match suggestions.is_empty() {
            true => match Self::calc_max_avg(&averages, curr_max_avg) {
                Some(next_max_avg) => Self::mk_suggestions(&board, &averages, next_max_avg),
                None => vec![],
            },
            false => suggestions,
        }
    }

    pub fn avg_col(&self) -> &[u16; 3] {
        &self.avg_col
    }

    pub fn avg_row(&self) -> &[u16; 3] {
        &self.avg_row
    }

    pub fn avg_tl_br(&self) -> u16 {
        self.avg_tl_br
    }

    pub fn avg_bl_tr(&self) -> u16 {
        self.avg_bl_tr
    }

    pub fn max_avg(&self) -> u16 {
        self.max_avg
    }

    pub fn suggestions(&self) -> &Vec<&'static BoardPosition> {
        &self.suggestions
    }

    pub fn from_board(board: &Board) -> Result<Recommendation, &'static str> {
        validate_board(&board)?;

        let eb = board.available_endings();

        let c0 = eb.avg_for_col(0);
        let c1 = eb.avg_for_col(1);
        let c2 = eb.avg_for_col(2);

        let r0 = eb.avg_for_row(0);
        let r1 = eb.avg_for_row(1);
        let r2 = eb.avg_for_row(2);

        let avg_tl_br = eb.avg_for_diag_row(DiagRow::TopLeftBottomRight);
        let avg_bl_tr = eb.avg_for_diag_row(DiagRow::BottomLeftTopRight);

        let avg_col = [c0, c1, c2];
        let avg_row = [r0, r1, r2];
        let averages = [c0, c1, c2, r0, r1, r2, avg_tl_br, avg_bl_tr];

        let max_avg = Self::calc_max_avg(&averages, u16::MAX).unwrap();
        // let sugg = Vec::with_capacity(1);
        let suggestions = Self::mk_suggestions(&board, &averages, max_avg);

        let out = Recommendation {
            avg_col,
            avg_row,
            avg_tl_br,
            avg_bl_tr,
            max_avg,
            suggestions,
        };

        Ok(out)
    }
}

