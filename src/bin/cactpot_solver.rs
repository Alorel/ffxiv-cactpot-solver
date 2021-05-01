use eframe;

use cactpot_solver::CactpotSolverGUI;

pub fn main() {
    let app = CactpotSolverGUI::default();
    eframe::run_native(Box::new(app));
}
