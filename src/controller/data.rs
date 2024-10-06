// Purpose: Data types for the controller module.
// these are just generic controller mappings from input to high-level types
// like up/down/left/right, etc.
// it does _not_ concern itself with actual controllers like jump/hide/shoot
// that is handled by the enemy controller, ui controller, etc.
use std::sync::atomic::{AtomicBool, AtomicI32};
use web_sys::KeyboardEvent;

use crate::{prelude::*, enemy::data::EnemyKind};

#[derive(Debug, Clone)]
pub enum Input {
    PointerDown(X, Y),
    PointerDrag(X, Y, DeltaX, DeltaY, DiffX, DiffY),
    PointerHover(X, Y),
    PointerUp(X, Y, DeltaX, DeltaY, DiffX, DiffY),
    PointerClick(X, Y),
    KeyDown(Key),
    KeyUp(Key),
    Wheel(WheelDeltaMode, WheelX, WheelY, WheelZ),
    ResetButton,
}


#[derive(Debug, Clone, Copy)]
pub enum WheelDeltaMode {
    Pixel,
    Line,
    Page
}

impl std::convert::TryFrom<u32> for WheelDeltaMode {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Pixel),
            1 => Ok(Self::Line),
            2 => Ok(Self::Page),
            _ => Err("unknown wheel delta mode!")
        }
    }
}

// can add more fields as-needed to map from
// https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.KeyboardEvent.html
#[derive(Debug, Clone)]
pub enum Key {
    Unknown(String),
    Space,
    Right,
    Left,
    Up,
    Down,
    Number1,
    Number2,
    Number3,
    Number4,
    ToggleDebugColliders,
    Pause,
}

impl From<&KeyboardEvent> for Key {
    fn from(evt:&KeyboardEvent) -> Self {
        let key_str = evt.key().to_lowercase();
        match key_str.as_str() {
            "space" | "spacebar" | " " => Self::Space,
            "d" | "l" | "arrowright" => Self::Right,
            "a" | "h" | "arrowleft" => Self::Left,
            "w" | "k" | "arrowup" => Self::Up,
            "s" | "j" | "arrowdown" => Self::Down,
            "p" => Self::Pause,
            "1" => Self::Number1, 
            "2" => Self::Number2, 
            "3" => Self::Number3, 
            "4" => Self::Number4, 
            "c" => Self::ToggleDebugColliders, 
            _ => Self::Unknown(key_str)
        }
    }
}
pub struct InputState {
    pub is_pointer_down: AtomicBool,
    pub first_pointer_move_x: AtomicI32,
    pub first_pointer_move_y: AtomicI32,
    pub last_pointer_move_x: AtomicI32,
    pub last_pointer_move_y: AtomicI32,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            is_pointer_down: AtomicBool::new(false),
            first_pointer_move_x: AtomicI32::new(0),
            first_pointer_move_y: AtomicI32::new(0),
            last_pointer_move_x: AtomicI32::new(0),
            last_pointer_move_y: AtomicI32::new(0),
        }
    }
}


//Delta is the change since the last move 
//Diff is the change since pointer down
type X = i32;
type Y = i32;
type DeltaX = i32;
type DeltaY = i32;
type DiffX = i32;
type DiffY = i32;
type WheelX = f64;
type WheelY = f64;
type WheelZ = f64;