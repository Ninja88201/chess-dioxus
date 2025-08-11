//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.

mod chessboard;
pub use chessboard::Chessboard;
mod promotion;
pub use promotion::Promotion;
mod pieces;
pub use pieces::Pieces;
mod moves;
pub use moves::Moves;
mod highlights;
pub use highlights::Highlights;
mod tabs;
pub use tabs::ChessTabs;
mod settings;

mod helper;
pub use helper::{get_asset, Circle, Square, EnginePlays};