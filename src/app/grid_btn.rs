use eframe::egui::{self, Button, Color32, Sense, Ui, Vec2};

use cactpot_solver_core::{Board, ValuedBoardPosition};

use crate::app::state::CactpotState;

const SIZE: Vec2 = Vec2::new(25.0, 25.0);
const SUGGESTED_COLOUR: Color32 = Color32::from_rgb(2, 125, 232);

fn resolve_sense(is_enabled: bool) -> Sense {
    match is_enabled {
        true => Sense::click(),
        false => Sense::hover()
    }
}

fn calc_is_enabled(board: &Board, pos: &ValuedBoardPosition) -> bool {
    board.len() < 4 && !board.contains_value(pos.value())
}

pub fn draw(
    ui: &mut Ui,
    state: &mut CactpotState,
    pos: &'static ValuedBoardPosition,
    is_suggested: bool,
) -> egui::Response {
    let is_enabled = calc_is_enabled(state.board(), &pos);
    let (rect, rsp) = ui.allocate_exact_size(SIZE, resolve_sense(is_enabled));

    let mut btn = Button::new(pos.value().to_string());
    if is_suggested && is_enabled {
        btn = btn.fill(Some(SUGGESTED_COLOUR))
    }
    ui.put(rect, btn.enabled(is_enabled));

    if rsp.clicked() {
        if let Err(e) = state.board_mut().fill(pos) {
            eprintln!("Failed to fill: {:?}", e);
        }
        state.update_recommendation();
    }

    rsp
}

