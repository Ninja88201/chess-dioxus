use chess_lib::{Colour, Piece, Tile};
use dioxus::prelude::*;

const PIECE_IMAGES: [[Asset; 6]; 2] = [
    // White
    [
        asset!("/assets/wP.svg"),
        asset!("/assets/wN.svg"),
        asset!("/assets/wB.svg"),
        asset!("/assets/wR.svg"),
        asset!("/assets/wQ.svg"),
        asset!("/assets/wK.svg"),
    ],
    // Black
    [
        asset!("/assets/bP.svg"),
        asset!("/assets/bN.svg"),
        asset!("/assets/bB.svg"),
        asset!("/assets/bR.svg"),
        asset!("/assets/bQ.svg"),
        asset!("/assets/bK.svg"),
    ],
];
#[derive(Clone, Copy, PartialEq)]
pub enum EnginePlays {
    None,
    White,
    Black,
    Both,
}
pub fn get_asset(colour: Colour, piece: Piece) -> Asset {
    match (colour, piece) {
        (Colour::White, Piece::Pawn) => PIECE_IMAGES[0][0],
        (Colour::White, Piece::Knight) => PIECE_IMAGES[0][1],
        (Colour::White, Piece::Bishop) => PIECE_IMAGES[0][2],
        (Colour::White, Piece::Rook) => PIECE_IMAGES[0][3],
        (Colour::White, Piece::Queen) => PIECE_IMAGES[0][4],
        (Colour::White, Piece::King) => PIECE_IMAGES[0][5],
        (Colour::Black, Piece::Pawn) => PIECE_IMAGES[1][0],
        (Colour::Black, Piece::Knight) => PIECE_IMAGES[1][1],
        (Colour::Black, Piece::Bishop) => PIECE_IMAGES[1][2],
        (Colour::Black, Piece::Rook) => PIECE_IMAGES[1][3],
        (Colour::Black, Piece::Queen) => PIECE_IMAGES[1][4],
        (Colour::Black, Piece::King) => PIECE_IMAGES[1][5],
    }
}
#[component]
pub fn Circle(tile: Tile, size: f32, flipped: Signal<bool>) -> Element {
    let radius = 12.5 * size;
    let (file, rank) = if *flipped.read() { tile.get_coords() } else { tile.mirror_tile().get_coords()};
    let left = file as f32 * 12.5 + (12.5 - radius) / 2.0;
    let top = rank as f32 * 12.5 + (12.5 - radius) / 2.0;
    rsx!(
        div {
            style: "position: absolute; width: {radius}%; height: {radius}%; left: {left}%; top: {top}%; \
                    border-radius: 50%; background-color: rgba(47, 47, 54, 0.5); pointer-events: none;"
        }
    )
}

#[component]
pub fn Square(tile: Tile, size: i32, colour: String, flipped: Signal<bool>) -> Element {
    let (file, rank) = if *flipped.read() { tile.get_coords() } else { tile.mirror_tile().get_coords()};
    let left = file as i32 * size;
    let top = rank as i32 * size;
    rsx!(
        div {
            style: "position: absolute; width: {size}px; height: {size}px; left: {left}px; top: {top}px; \
                    background-color: {colour}; pointer-events: none; z-index: 0;"
        }
    )
}