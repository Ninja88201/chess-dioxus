use chess_lib::{Board, Colour, Piece, Tile};
use dioxus::prelude::*;
use crate::components::get_asset;


#[component]
pub fn Promotion(from: Tile, to: Tile, board: Signal<Board>, flipped: Signal<bool>) -> Element {
    let colour = if to.is_promotion(Colour::White) { Colour::White} else { Colour::Black };
    let (file, rank) = to.get_coords();
    let mut b = board.clone();
    rsx!(
        div {
            style: format!("
                position: absolute;
                left: {}%;
                top: {}%;
                background: white;
                border: 1px solid black;
                display: flex;
                flex-direction: column;
                z-index: 10;
            ", file as f32 * 12.5, rank as f32 * 12.5),

            for piece in Piece::PROMOTION_PIECES {
                {
                    let asset = get_asset(colour, piece);
                    rsx!(
                        div {
                            key: "promo-{piece:?}",
                            onclick: move |_| {
                                let _ = b.write().try_move_piece(from, to, Some(piece));
                            },
                            img { src: "{asset}" },
                        }
                    )
                }
            }
        }
    )
}