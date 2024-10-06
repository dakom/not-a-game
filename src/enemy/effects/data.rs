use crate::prelude::*;

#[derive(Component, Debug)]
pub struct EnemyEffect {
    hiding_value: f32,
    hiding_multiplier: f32,
}

impl EnemyEffect {
    pub const HIDING_MIN: f32 = 0.2;
    pub const HIDING_MAX: f32 = 1.0;
    pub const HIDING_UPDATE: f32 = 0.1;

    pub fn new() -> Self {
        Self {
            hiding_value: Self::HIDING_MIN,
            hiding_multiplier: 1.0,
        }
    }

    pub fn update_hiding(&mut self) -> f32 {
        let value = (self.hiding_value + (Self::HIDING_UPDATE * self.hiding_multiplier))
            .min(Self::HIDING_MAX)
            .max(Self::HIDING_MIN);
        if value >= Self::HIDING_MAX {
            self.hiding_multiplier = -1.0;
        } else if value <= Self::HIDING_MIN {
            self.hiding_multiplier = 1.0;
        }
        self.hiding_value = value;

        value
    }
}
