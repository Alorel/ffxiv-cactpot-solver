use eframe::egui::{self, Align, Direction, Label, Layout, Sense, Ui, Vec2};

use cactpot_solver_core::BoardPosition;

use crate::app::grid_cell;
use crate::app::state::CactpotState;

const SPACING: Vec2 = Vec2::new(10.0, 10.0);

mod print_tip {
    use eframe::egui::{Color32, Label};

    use cactpot_solver_core::Recommendation;

    use super::{grid_cell, Align, CactpotState, Direction, Layout, Sense, Ui, Vec2};

    const TIP_SIZE_ROW: Vec2 = Vec2::new(50.0, grid_cell::SIZE.y);
    pub const TIP_SIZE_COL: Vec2 = Vec2::new(grid_cell::SIZE.x, 14.0);
    const SUGGESTION_COLOUR: Color32 = Color32::from_rgb(153, 152, 151);
    const SUGGESTION_BEST_COLOUR: Color32 = Color32::from_rgb(9, 209, 2);

    #[inline]
    fn resolve_colour<T: PartialEq<T>>(tested_value: T, avg_value: T) -> Color32 {
        if avg_value == tested_value {
            SUGGESTION_BEST_COLOUR
        } else {
            SUGGESTION_COLOUR
        }
    }

    pub fn row(ui: &mut Ui, row: usize, state: &CactpotState) {
        let (rect, _) = ui.allocate_exact_size(TIP_SIZE_ROW, Sense::hover());

        ui.allocate_ui_at_rect(rect, |ui| {
            if let Some(recommendation) = state.recommendation() {
                let lbl = {
                    let avg = recommendation.avg_row()[row];
                    Label::new(avg.to_string())
                        .text_color(resolve_colour(avg, recommendation.max_avg()))
                };
                ui.add(lbl);
            }
        });
    }

    pub fn col(ui: &mut Ui, col: usize, recommendation: &Recommendation) {
        let lbl = {
            let avg = recommendation.avg_col()[col];
            Label::new(avg.to_string()).text_color(resolve_colour(avg, recommendation.max_avg()))
        };
        ui.add_sized(TIP_SIZE_COL, lbl);
    }

    pub fn diag(ui: &mut Ui, avg: u16, max_avg: u16, dir: Direction) {
        let layout = Layout::from_main_dir_and_cross_align(dir, Align::Center);
        ui.allocate_ui_with_layout(TIP_SIZE_COL, layout, |ui| {
            ui.add(Label::new(avg.to_string()).text_color(resolve_colour(avg, max_avg)));
        });
    }
}

fn draw_main(ui: &mut Ui, state: &mut CactpotState) {
    for row in 0u8..3 {
        ui.horizontal(|ui| {
            ui.allocate_exact_size(print_tip::TIP_SIZE_COL, Sense::hover());
            for col in 0u8..3 {
                grid_cell::draw(ui, state, BoardPosition::new(col, row));
            }
            print_tip::row(ui, row as usize, state);
        });
    }
}

fn draw_bottom(ui: &mut Ui, state: &CactpotState) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = SPACING;

        match state.recommendation() {
            Some(recommendation) => {
                print_tip::diag(
                    ui,
                    recommendation.avg_bl_tr(),
                    recommendation.max_avg(),
                    Direction::RightToLeft,
                );
                for col in 0..3 {
                    print_tip::col(ui, col, recommendation);
                }
                print_tip::diag(
                    ui,
                    recommendation.avg_tl_br(),
                    recommendation.max_avg(),
                    Direction::LeftToRight,
                );
            }
            None => {
                ui.add_sized(print_tip::TIP_SIZE_COL, Label::new(""));
            }
        }
    });
}

pub fn draw(ui: &mut Ui, state: &mut CactpotState) -> egui::Response {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = SPACING;
        draw_main(ui, state);
        draw_bottom(ui, state);
    })
    .response
}
