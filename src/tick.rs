use crate::prelude::*;

pub type BeginTickViewMut<'a> = UniqueViewMut<'a, BeginTick>;
pub type BeginTickView<'a> = UniqueView<'a, BeginTick>;

// the tick info for the "begin" phase of mainloop
#[derive(Component, Unique, Default)]
pub struct BeginTick {
    pub time: f64,
    pub delta: f64,
}

pub type UpdateTickViewMut<'a> = UniqueViewMut<'a, UpdateTick>;
pub type UpdateTickView<'a> = UniqueView<'a, UpdateTick>;

// the tick info for the "update" phase of mainloop
#[derive(Component, Unique, Default)]
pub struct UpdateTick {
    pub delta: f64,
    pub viewport_width: f64,
    pub viewport_height: f64,
}

pub type DrawTickViewMut<'a> = UniqueViewMut<'a, DrawTick>;
pub type DrawTickView<'a> = UniqueView<'a, DrawTick>;

// the tick info for the "draw" phase of mainloop
#[derive(Component, Unique, Default)]
pub struct DrawTick {
    pub interpolation: f64,
    pub viewport_width: f64,
    pub viewport_height: f64,
}

pub type EndTickViewMut<'a> = UniqueViewMut<'a, EndTick>;
pub type EndTickView<'a> = UniqueView<'a, EndTick>;

// the tick info for the "end" phase of mainloop
#[derive(Component, Unique, Default)]
pub struct EndTick {
    pub fps: f64,
    pub abort: bool,
}

// Not really a specific ticker, but relevant to the tick system
pub type PauseTickViewMut<'a> = UniqueViewMut<'a, PauseTick>;
pub type PauseTickView<'a> = UniqueView<'a, PauseTick>;

#[derive(Component, Unique, PartialEq, Clone, Debug)]
pub enum PauseTick {
    Welcome,
    Help,
    Running,
    ManuallyPaused,
    LostVisibility {
        timestamp: f64,
        previous: Box<PauseTick>,
    },
    GameOver {},
}
