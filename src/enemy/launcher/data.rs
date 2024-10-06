use crate::{prelude::*, spritesheet::SpriteSheet, renderer::Renderer};

#[derive(Component)]
pub struct EnemyLauncher {
    pub spritesheet: SpriteSheet,
    pub launching: bool,
    pub side: LauncherSide,
}

impl EnemyLauncher {
    pub fn new(side: LauncherSide, spritesheet: SpriteSheet) -> Self {
        Self {
            spritesheet,
            launching: false,
            side
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LauncherSide {
    Left,
    Right,
}
