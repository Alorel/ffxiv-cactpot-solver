use eframe::egui::{self, Direction, Layout, Ui, Vec2};

use cactpot_solver_core::{Board, BoardPosition, ValuedBoardPosition};

use crate::app::state::CactpotState;

use super::grid_btn;

pub const SIZE: Vec2 = Vec2::new(79.0, 79.0);
const SPACING: Vec2 = Vec2::new(2.0, 2.0);

fn draw_filled(target_ui: &mut Ui, board: &Board, pos: BoardPosition) -> egui::Response {
    target_ui
        .allocate_ui_with_layout(
            SIZE,
            Layout::centered_and_justified(Direction::LeftToRight),
            |ui| {
                if let Some(v) = board.find(pos) {
                    ui.heading(v.value().to_string());
                }
            },
        )
        .response
}

fn draw_unfilled(ui: &mut Ui, state: &mut CactpotState, cell_pos: BoardPosition) -> egui::Response {
    let is_suggested = match state.recommendation() {
        Some(v) => v.suggestions().contains(&cell_pos),
        None => false,
    };

    ui.allocate_ui(SIZE, |ui| {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing = SPACING;

            for row in 0u8..3 {
                let value = (row * 3) + 1;

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = SPACING;

                    for col in 0u8..3 {
                        let vbs = ValuedBoardPosition::from_pos(value + col, cell_pos);
                        grid_btn::draw(ui, state, vbs, is_suggested);
                    }
                });
            }
        });
    })
    .response
}

pub fn draw(ui: &mut Ui, state: &mut CactpotState, cell_pos: BoardPosition) -> egui::Response {
    if state.board().contains_position(cell_pos) {
        draw_filled(ui, state.board(), cell_pos)
    } else {
        draw_unfilled(ui, state, cell_pos)
    }
}
