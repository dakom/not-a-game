use crate::{prelude::*, config::CONFIG};

pub type BomberViewMut<'a> = UniqueViewMut<'a, Bomber>;
pub type BomberView<'a> = UniqueView<'a, Bomber>;

#[derive(Component, Unique)]
pub struct Bomber {
    pub drop_countdown: Option<f64>
}

impl Bomber {
    pub fn new() -> Self {
        Self {
            //drop_countdown: None 
            drop_countdown: Some(CONFIG.initial_drop_countdown)
        }
    }
}