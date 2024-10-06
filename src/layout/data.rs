use std::ops::{Deref, DerefMut};

use crate::prelude::*;

// LayoutPosition is normalized and places things in a *relative* position on the screen
#[derive(Component, Debug)]
pub struct LayoutPosition {
    _values: Vec3
}

impl LayoutPosition {
    pub fn new(values: Vec3) -> Self {
        Self {
            _values: values
        }
    }
}


impl Deref for LayoutPosition {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self._values
    }
}

impl DerefMut for LayoutPosition { 
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._values
    }
}

// LayoutAnchor is not normalized and places things in an *absolute* position on the screen 
// It's also used as an offset, e.g. to nudge things over by their original bitmap size
#[derive(Component, Debug)]
pub struct LayoutAnchor {
    _values: Vec3
}

impl LayoutAnchor {
    pub fn new(values: Vec3) -> Self {
        Self {
            _values: values
        }
    }
}


impl Deref for LayoutAnchor {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self._values
    }
}

impl DerefMut for LayoutAnchor { 
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._values
    }
}