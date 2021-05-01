use cactpot_solver_core::{Board, Recommendation};

pub struct CactpotState {
    board: Board,
    recommendation: Option<Recommendation>,
}

impl CactpotState {
    pub fn board(&self) -> &Board {
        &self.board
    }
    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }
    pub fn recommendation(&self) -> &Option<Recommendation> {
        &self.recommendation
    }
    pub fn clear_recommendation(&mut self) {
        self.recommendation = None;
    }

    pub fn update_recommendation(&mut self) {
        self.recommendation = Recommendation::from_board(&self.board)
            .map(|v| Some(v))
            .unwrap_or(None);
    }
}

impl Default for CactpotState {
    fn default() -> Self {
        Self {
            board: Board::default(),
            recommendation: None,
        }
    }
}
