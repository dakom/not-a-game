use std::ops::{Deref, DerefMut};

use nalgebra::{Point3, Point};
use nalgebra_glm::{Vec2, Vec4, normalize};
use web_sys::WebGlQuery;

use crate::{prelude::*, spritesheet::SpriteSheet, renderer::uvs::Uvs, camera::Camera};

#[derive(Component, Debug, Default)]
pub struct Collider {
    // hardcoded to 8 because we're representing a rect for now
    // i.e. 4 vertices, each with an x and y
    pub vertices: [f32;8],

    pub height: f32,
    pub width: f32,
}

impl Collider {
    pub fn update(&mut self, width: f32, height: f32, transform: &Mat4) {
        // Transform the collider's vertices by the world transform (e.g. model matrix... not view or projection)
        let point = transform.transform_point(&Point3::new(0.0, height, 0.0));
        // left,top
        self.vertices[0] = point.x;
        self.vertices[1] = point.y;

        let point = transform.transform_point(&Point3::new(0.0, 0.0, 0.0));
        // left,bottom
        self.vertices[2] = point.x;
        self.vertices[3] = point.y;

        let point = transform.transform_point(&Point3::new(width, height, 0.0));
        // right,top
        self.vertices[4] = point.x;
        self.vertices[5] = point.y;

        let point = transform.transform_point(&Point3::new(width, 0.0, 0.0));
        // right,bottom
        self.vertices[6] = point.x;
        self.vertices[7] = point.y;

        self.width = width;
        self.height = height;
    }

    pub fn center(&self) -> Vec2 {
        let left = self.vertices[0].min(self.vertices[2]).min(self.vertices[4]).min(self.vertices[6]);
        let right = self.vertices[0].max(self.vertices[2]).max(self.vertices[4]).max(self.vertices[6]);
        let bottom = self.vertices[1].min(self.vertices[3]).min(self.vertices[5]).min(self.vertices[7]);
        let top = self.vertices[1].max(self.vertices[3]).max(self.vertices[5]).max(self.vertices[7]);

        let x = left + ((right - left).abs() / 2.0);
        let y = bottom + ((top - bottom).abs() / 2.0);

        Vec2::new(left, bottom)
    }


}

pub type CollisionEventQueueViewMut<'a> = NonSendSync<UniqueViewMut<'a, CollisionEventQueue>>;
pub type CollisionEventQueueView<'a> = NonSendSync<UniqueView<'a, CollisionEventQueue>>;
#[derive(Unique, Component, Default)]
pub struct CollisionEventQueue {
    queue: Vec<CollisionEvent>,
}

impl CollisionEventQueue {
    pub fn has_any_collision(&self, entity_a: EntityId, entity_b: EntityId) -> bool {
        self.queue.iter().any(|e| (e.a.entity == entity_a && e.b.entity == entity_b) || (e.b.entity == entity_a && e.a.entity == entity_b))
    }
}

impl CollisionEventQueue {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
        }
    }
}

impl Deref for CollisionEventQueue {
    type Target = Vec<CollisionEvent>;

    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}

impl DerefMut for CollisionEventQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.queue
    }
}

pub struct CollisionEvent {
    pub a: CollisionEventTarget,
    pub b: CollisionEventTarget,
    pub occlusion_query: Option<WebGlQuery>,

}

#[derive(Debug)]
pub struct CollisionEventTarget {
    pub entity: EntityId,
    pub vertices: [f32;8],
    pub uvs: [f32;8],
    pub texture_id: Id,
}