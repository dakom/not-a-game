#[derive(Clone, Debug, Default)]
pub struct Bounds {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Bounds {
    // assumes x is right
    pub fn right(&self) -> f64 {
        self.x + self.width
    }

    pub fn center_x(&self) -> f64 {
        self.x + (self.width / 2.0)
    }
    // assumes y is up
    pub fn top(&self) -> f64 {
        self.y + self.height
    }
}

pub trait BoundsExt {
    fn get_bounds(&self) -> Bounds;
}
