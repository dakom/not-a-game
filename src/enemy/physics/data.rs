use std::ops::{Deref, DerefMut};

use crate::prelude::*;

// this is in the coordinate system of bottom-left corner
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EnemyDirection {
    Right,
    Left,
}
