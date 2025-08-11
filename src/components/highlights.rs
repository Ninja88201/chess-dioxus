use chess_lib::{Board, Colour, Tile};
use dioxus::prelude::*;

use crate::components::helper::Square;

#[component]
pub fn Highlights(board: Signal<Board>, selected: Signal<Option<Tile>>, flipped: Signal<bool>) -> Element {
    let w_king = board.read().white.king_tile();
    let b_king = board.read().black.king_tile();

    let white_check = board.read().is_in_check(Colour::White);
    let black_check = board.read().is_in_check(Colour::Black);
    rsx!(
        if let Some(wkt) = w_king{
            if white_check {
                Square {  tile: wkt, size: 60, colour: "rgba(255, 25, 25, 1)", flipped }
            }
        }
        if let Some(bkt) = b_king{
            if black_check {
                Square {  tile: bkt, size: 60, colour: "rgba(255, 25, 25, 1)", flipped }
            }
        }
    )
}