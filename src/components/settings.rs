use chess_lib::Board;
use dioxus::prelude::*;

use crate::components::EnginePlays;

#[component]
pub fn Settings(board: Signal<Board>, flipped: Signal<bool>, engine: Signal<EnginePlays>) -> Element {
    rsx!(
        div { 
            style: "display: flex; justify-content: center; align-items: center;",
            select {
                onchange: move |e| {
                    let value = e.value();
                    let variant = match value.as_str() {
                        "White" => EnginePlays::White,
                        "Black" => EnginePlays::Black,
                        "Both" => EnginePlays::Both,
                        _ => EnginePlays::None,
                    };
                    engine.set(variant);
                },
                option { value: "None", "None" }
                option { value: "White", "White" }
                option { value: "Black", "Black" }
                option { value: "Both", "Both" }
            }
            button { onclick: move |_| flipped.toggle(), p { "Flip Board" }}
            button { onclick: move |_| board.write().undo_move(), p { "Undo Move"} }
        }
    )
}