use std::fmt::Debug;

use crate::utils::bounds::Bounds;

pub struct Uvs {
    pub data: [f32; 8],
}

impl Uvs {
    pub fn flip_x(&self) -> Self {
        let tl_x = self.data[0];
        let tl_y = self.data[1];
        let bl_x = self.data[2];
        let bl_y = self.data[3];
        let tr_x = self.data[4];
        let tr_y = self.data[5];
        let br_x = self.data[6];
        let br_y = self.data[7];

        Self {
            data: [tr_x, tr_y, br_x, br_y, tl_x, tl_y, bl_x, bl_y],
        }
    }

    pub fn new(atlas_width: f32, atlas_height: f32, bounds: &Bounds) -> Uvs {
        let atlas_width = atlas_width as f64;
        let atlas_height = atlas_height as f64;

        let Bounds {
            x,
            y,
            width,
            height,
        } = *bounds;

        //Bounds are assuming 0,0 is bottom-left
        //Texture atlas assumes 0,0 is top-right
        //So we need to invert the y axis
        let mut x1 = x;
        let mut y1 = atlas_height - (y + height);
        let mut x2 = x + width;
        let mut y2 = y1 + height;

        //Normalize relative to full image width/height
        x1 /= atlas_width;
        y1 /= atlas_height;
        x2 /= atlas_width;
        y2 /= atlas_height;

        //Get the corners, just for the sake of clarity
        //Might as well do the casting here too
        let bl = (x1 as f32, y1 as f32);
        let tl = (x1 as f32, y2 as f32);
        let br = (x2 as f32, y1 as f32);
        let tr = (x2 as f32, y2 as f32);

        //return it as a straight array
        Self {
            data: [tl.0, tl.1, bl.0, bl.1, tr.0, tr.1, br.0, br.1],
        }
    }
}

impl Debug for Uvs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Uvs")
            .field("tl_x", &self.data[0])
            .field("tl_y", &self.data[1])
            .field("bl_x", &self.data[2])
            .field("bl_y", &self.data[3])
            .field("tr_x", &self.data[4])
            .field("tr_y", &self.data[5])
            .field("br_x", &self.data[6])
            .field("br_y", &self.data[7])
            .finish()
    }
}
