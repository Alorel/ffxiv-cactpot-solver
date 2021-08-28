use cactpot_solver_core::{Board, Recommendation};

#[derive(Default)]
pub struct CactpotState {
    board: Board,
    recommendation: Option<Recommendation>,
}

impl CactpotState {
    #[inline]
    pub fn board(&self) -> &Board {
        &self.board
    }

    #[inline]
    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    #[inline]
    pub fn recommendation(&self) -> &Option<Recommendation> {
        &self.recommendation
    }

    #[inline]
    pub fn clear_recommendation(&mut self) {
        self.recommendation = None;
    }

    pub fn update_recommendation(&mut self) {
        self.recommendation = Recommendation::from_board(&self.board).ok();
    }
}
