use godot::prelude::*;

static mut SCORE1: i32 = 0;
static mut SCORE2: i32 = 0;

struct Pong1;

mod ball;
mod paddles;
mod score;

#[gdextension]
unsafe impl ExtensionLibrary for Pong1 {}
