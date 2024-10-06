use std::ops::{Deref, DerefMut};

use rand::rngs::ThreadRng;
use crate::prelude::*;

pub type RandView<'a> = NonSendSync<UniqueView<'a, Rand>>;
pub type RandViewMut<'a> = NonSendSync<UniqueViewMut<'a, Rand>>;

#[derive(Component, Unique)]
pub struct Rand {
    inner: ThreadRng
}

impl Rand {
    pub fn new() -> Self {
        Self {
            inner: rand::thread_rng()
        }
    }
}

impl Deref for Rand {
    type Target = ThreadRng;

    fn deref(&self) -> &ThreadRng{
        &self.inner
    }
}

impl DerefMut for Rand {
    fn deref_mut(&mut self) -> &mut ThreadRng{
        &mut self.inner
    }
}