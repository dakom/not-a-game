use crate::{config::CONFIG, prelude::*, spritesheet::{self, SpriteSheet}};

#[derive(Component, Debug, Clone, Default)]
pub struct Animation {
    pub index: usize,
    pub timeout: Option<f64>,
    pub len: usize,
    pub cell_duration: f64
}

impl Animation {
    pub fn new(spritesheet: &SpriteSheet) -> Self {
        let mut _self = Self::default();
        _self.reset(spritesheet);
        _self
    }

    pub fn reset(&mut self, spritesheet: &SpriteSheet) {
        self.index = 0;
        self.timeout = Some(spritesheet.cell_duration);
        self.len = spritesheet.cells.len();
        self.cell_duration = spritesheet.cell_duration;

    }
}