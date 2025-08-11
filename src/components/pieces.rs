use chess_lib::{Board, Colour, Piece};
use dioxus::prelude::*;
use crate::components::get_asset;


#[component]
pub fn Pieces(board: Signal<Board>, flipped: Signal<bool>) -> Element {
    // White pieces
    rsx!(
        for piece in Piece::ALL_PIECES {
            for tile in board.read().white.bb[piece as usize] {
                {
                    let mut index = tile.to_u8();
                    let (file, mut rank) = tile.get_coords();
                    if !*flipped.read() {
                        index = 64 - index;
                        // file = 7 - file;
                        rank = 7 - rank;
                    }

                    
                    let asset = get_asset(Colour::White, piece);
                    
                    rsx!(
                        img {
                            key: "w-{index}",
                            src: "{asset}",
                            style: "
                            position: absolute;
                            width: 12.5%;
                            height: 12.5%;
                            left: {file as f32 * 12.5}%;
                            top: {rank as f32 * 12.5}%;
                            pointer-events: none;
                            z-index: 1;
                            "
                        }
                    )
                }
            }
        }
        
        // Black pieces
        for piece in Piece::ALL_PIECES {
            for tile in board.read().black.bb[piece as usize] {
                {
                    let mut index = tile.to_u8();
                    let (file, mut rank) = tile.get_coords();
                    if !*flipped.read() {
                        index = 64 - index;
                        // file = 7 - file;
                        rank = 7 - rank;
                    }
                    
                    let asset = get_asset(Colour::Black, piece);
                    
                    rsx!(
                        img {
                            key: "b-{index}",
                            src: "{asset}",
                            style: "
                            position: absolute;
                            width: 12.5%;
                            height: 12.5%;
                            left: {file as f32 * 12.5}%;
                            top: {rank as f32 * 12.5}%;
                            pointer-events: none;
                            z-index: 1;
                            "
                        }
                    )
                }
            }
        }
    )
}