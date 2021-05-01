use eframe::egui::{CentralPanel, CtxRef, Direction, Label, Layout, Ui, Vec2};
use eframe::epi::{App, Frame, IconData};

use crate::app::state::CactpotState;

pub(crate) const WINDOW_SIZE: Vec2 = Vec2::new(440.0, 350.0);

pub(crate) mod grid_cell;
pub(crate) mod grid_btn;
pub(crate) mod grid;
pub(crate) mod state;

pub struct CactpotSolverGUI {
    state: CactpotState,
}

impl CactpotSolverGUI {
    fn draw_suggestions(&self, ui: &mut Ui) {
        let txt = match self.state.board().len() {
            0 => "Select the number the game's chosen for you",
            4 => "Good luck!",
            _ => "Pick the next number"
        };
        ui.add_sized(Vec2::new(WINDOW_SIZE.x, 14.0), Label::new(txt));
    }

    fn draw_controls(&mut self, ui: &mut Ui) {
        ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
            if ui.button("Reset").clicked() {
                self.state.board_mut().clear_fills();
                self.state.clear_recommendation();
            }
        });
    }
}

impl App for CactpotSolverGUI {
    fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>) {
        CentralPanel::default().show(&ctx, |ui| {
            grid::draw(ui, &mut self.state);

            ui.allocate_ui(Vec2::new(WINDOW_SIZE.x, 0.0), |ui| {
                ui.separator();
                self.draw_suggestions(ui);
                self.draw_controls(ui);
            });
        });
    }

    fn name(&self) -> &str {
        "Catcpot Solver"
    }

    fn initial_window_size(&self) -> Option<Vec2> {
        Some(WINDOW_SIZE)
    }

    fn is_resizable(&self) -> bool {
        false
    }

    fn icon_data(&self) -> Option<IconData> {
        let rgba = match image::load_from_memory(include_bytes!("mgp.png")) {
            Ok(img) => img.into_rgba8().into_vec(),
            Err(e) => {
                eprintln!("Failed to load app icon: {}", e);

                return None;
            }
        };
        let icon = IconData {
            rgba,
            height: 20,
            width: 20,
        };

        Some(icon)
    }
}

impl Default for CactpotSolverGUI {
    fn default() -> Self {
        Self {
            state: CactpotState::default(),
        }
    }
}
