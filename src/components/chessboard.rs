use chess_lib::{Board, MoveError, MoveResult, Tile};
use dioxus::prelude::*;
use crate::components::{settings::Settings, EnginePlays, Highlights, Moves, Pieces, Promotion};

#[component]
pub fn Chessboard(board: Signal<Board>, flipped: Signal<bool>, engine: Signal<EnginePlays>) -> Element {
    let mut selected = use_signal(|| None::<Tile>);
    let mut promotion = use_signal(|| None::<Tile>);


    rsx! {
        div {
            id: "chessboard",
            style: "
                position: relative;
                width: 480px;
                height: 480px;
                margin-left: auto;
                margin-right: auto;
                display: grid;
                grid-template-columns: repeat(8, 1fr);
                grid-template-rows: repeat(8, 1fr);
                border: 2px solid black;
            ",

            // Board squares
            for i in 0..64usize {
                {
                    let mut tile = Tile::new_unchecked(i as u8);
                    if !*flipped.read() {
                        tile = tile.mirror_tile();
                    }
                    let light = (i + i / 8) % 2 == if *flipped.read() { 1 } else { 0 };
                    let color = if light { "#f0d9bb" } else { "#b58863" };

                    let on_click = {
                        // let selected_clone = selected.clone();
                        let mut board = board.clone();
                        move |_| {
                            let s = *selected.read();
                            if let Some(from) = s {
                                if from != tile {
                                    let result = board.write().try_move_piece(from, tile, None);
                                    match result {
                                        Ok(m) => {
                                            selected.set(None);
                                            if let MoveResult::PromotionNeeded(t) = m {
                                                promotion.set(Some(t));
                                            }
                                        }
                                        Err(e) => {
                                            match e {
                                                MoveError::NoPieceSelected |
                                                MoveError::FriendlyCapture => selected.set(Some(tile)),
                                                MoveError::SameTile => selected.set(None),
                                                _ => {}
                                            }
                                        }
                                    }
                                } else {
                                    selected.set(None);
                                }
                            } else {
                                if board.read().current_players().0.pieces.get_bit(tile) {
                                    selected.set(Some(tile));
                                }
                            }
                        }
                    };

                    rsx!(
                        div {
                            key: "sq-{i}",
                            style: "background: {color}; width: 100%; height: 100%;",
                            onclick: on_click,
                        }
                    )
                }
            }
            Highlights { board, selected, flipped }
            Pieces { board, flipped }
            if let Some(s) = *selected.read() {
                Moves { board, selected: s, flipped }
            }
            if let Some(t) = *promotion.read() {
                Promotion { from: selected.read().unwrap(), to: t, board, flipped}
            }
        }
        div {
            p { 
                style: "font-size: 12px;
                        font-family: Consolas, monaco, monospace;
                        text-align: center", 
                "Fen: {board.read().to_fen()}"}
        }
    }
}