use crate::hitable::{ Hitable, Sphere };
use crate::ray::{ Point, zero_point };

#[derive(Copy, Clone)]
pub struct ViewPlane {
   pub hres: u32,
   pub vres: u32,
   pub pixel_size: f32,
}

pub struct World {
   pub view_plane: ViewPlane,
   pub objects: Vec<Box<dyn Hitable>>,
}

impl World {
    pub fn demo() -> World {
        let mut w = World {
            view_plane: ViewPlane { hres: 200, vres: 100, pixel_size: 1.0 },
            objects: Vec::new(),
        };
        w.objects.push(Box::new(Sphere { center: zero_point(), radius: 15.0 }));
        w.objects.push(Box::new(Sphere { center: Point::new(100.0, 4.0, -2.0), radius: 21.0 }));
        w
    }
}