use cactpot_solver_core::*;

fn fill(board: &mut Board, v: u8, col: u8, row: u8) {
    let pos = ValuedBoardPosition::from_u8(v, col, row);
    if let Err(e) = board.fill(pos) {
        panic!("Failed to fill: {:#?}", e);
    }

    match Recommendation::from_board(&board) {
        Err(e) => panic!("{:?}", e),
        Ok(r) => {
            // println!("{:#?}", r);
            for p in r.suggestions().iter() {
                println!("{}", p);
            }
        }
    };
}

pub fn main() {
    let dur = std::time::SystemTime::now();
    let mut board = Board::default();
    fill(&mut board, 5, 2, 1);
    println!("---");
    fill(&mut board, 4, 1, 1);
    println!("---");
    fill(&mut board, 2, 0, 2);
    println!("---");
    fill(&mut board, 1, 0, 0);

    println!(
        "Dur: {}s",
        (dur.elapsed().unwrap().as_millis() as f64) / 1000.0f64
    )
}
