#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_truncation)]
mod vm;
use macroquad::audio::Sound;
use std::sync::OnceLock;
pub use vm::OpCode;
pub use vm::VM;

pub const STACK_SIZE: usize = 16;
pub const SCREEN_WIDTH: u32 = 64;
pub const SCREEN_HEIGHT: u32 = 32;
pub const WINDOW_WIDTH: u32 = 1024;
pub const WINDOW_HEIGHT: u32 = 512;
pub const PIXEL_WIDTH: f32 = WINDOW_WIDTH as f32 / SCREEN_WIDTH as f32;
pub const PIXEL_HEIGHT: f32 = WINDOW_HEIGHT as f32 / SCREEN_HEIGHT as f32;
pub static SOUND: OnceLock<Sound> = OnceLock::new();
