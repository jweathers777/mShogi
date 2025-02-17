pub mod board;
pub mod r#move;
pub mod move_generator;
pub mod notator;
pub mod piece;
pub mod player;
pub mod variant;

#[cfg(test)]
mod board_tests;
mod move_notation_tests;
mod move_generator_tests;
mod variant_tests;
