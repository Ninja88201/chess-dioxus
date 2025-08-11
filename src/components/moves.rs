use chess_lib::{Board, MoveList, Tile};
use dioxus::prelude::*;

use crate::components::Circle;

#[component]
pub fn Moves(board: Signal<Board>, selected: Tile, flipped: Signal<bool>) -> Element {
    let mut moves = MoveList::new();
    board.read().generate_legal_moves_from(selected, &mut moves);
    rsx!(
        div {
            for m in moves.iter() {
                Circle { tile: m.to(), size: 0.5, flipped: flipped }
            }
        }
    )
}