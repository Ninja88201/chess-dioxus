use chess_engine::search::find_best_move;
use chess_lib::Board;
use dioxus::prelude::*;

use crate::components::{settings::Settings, Chessboard, EnginePlays};

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Play,
    PositionCreator,
}

#[component]
pub fn ChessTabs() -> Element {
    let mut active_tab = use_signal(|| Tab::Play);

    rsx!(
        div {
            style: "width: 100%; margin: 0 auto; font-family: sans-serif;",

            // Tab buttons
            div {
                style: "
                    display: flex;
                    border-bottom: 2px solid #ccc;
                ",
                button {
                    style: tab_button_style(*active_tab.read() == Tab::Play),
                    onclick: move |_| active_tab.set(Tab::Play),
                    "Play"
                }
                button {
                    style: tab_button_style(*active_tab.read() == Tab::PositionCreator),
                    onclick: move |_| active_tab.set(Tab::PositionCreator),
                    "Position Creator"
                }
            }

            // Tab content
            div {
                style: "padding: 1rem;",
                match *active_tab.read() {
                    Tab::Play => rsx!( PlayTab {} ),
                    Tab::PositionCreator => rsx!( PositionCreatorTab {} ),
                }
            }
        }
    )
}

fn tab_button_style(active: bool) -> String {
    let colour = "#8d8888ff";
    if active {
        format!("flex: 1; padding: 0.5rem; border: none; background: #00ccffa2; font-weight: bold; cursor: pointer; color: {colour}").to_string()
    } else {
        format!("flex: 1; padding: 0.5rem; border: none; background: #000000ff; cursor: pointer; color: {colour}").to_string()
    }
}

fn engine_turn(board: Signal<Board>, engine: Signal<EnginePlays>) -> bool {
    let turn = board.read().turn;
    let e = *engine.read();
    if e == EnginePlays::None {
        return false;
    }
    if e == EnginePlays::Both {
        return true;
    }
    if (turn.white() && e == EnginePlays::White) || (turn.black() && e == EnginePlays::Black) {
        return true;
    }
    false
}

#[component]
fn PlayTab() -> Element {
    let mut board = use_signal(Board::new);
    let flipped = use_signal(|| false);
    let engine = use_signal(|| EnginePlays::None);
    if engine_turn(board, engine) {
        let best = find_best_move(&mut board.write(), 6);
        if let Some(m) = best {
            board.write().make_move_unchecked(m);
        }
    }
    rsx!(
        Chessboard { board, flipped, engine }
        Settings { board, flipped, engine }
    )
}
#[component]
fn PositionCreatorTab() -> Element {
    let board = use_signal(Board::new);
    let flipped = use_signal(|| false);
    let engine = use_signal(|| EnginePlays::None);

    rsx!(
        Chessboard { board, flipped, engine }
    )
}