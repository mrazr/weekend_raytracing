use crate::hitable::{ Hitable, Sphere, Plane };
use crate::ray::{ Point, zero_point, Vec3 };
use crate::material::Material;
use cgmath::Zero;

#[derive(Copy, Clone)]
pub struct ViewPlane {
   pub hres: u32,
   pub vres: u32,
   pub pixel_size: f32,
   pub z: f32,
}

pub struct World {
   pub view_plane: ViewPlane,
   pub objects: Vec<Box<dyn Hitable>>,
   pub background_color: Vec3,
}

impl World {
    pub fn demo() -> World {
        let mut w = World {
            view_plane: ViewPlane { hres: 200, vres: 200, pixel_size: 1.0, z: 100.0 },
            objects: Vec::new(),
            background_color: Vec3::new(0.5, 0.5, 0.5),
        };
        let mat1 = Material { color: Vec3::new(1.0, 1.0, 0.0) };
        let mat2 = Material { color: Vec3::new(0.0, 1.0, 0.5) };
        let mat3 = Material { color: Vec3::new(0.0, 0.0, 1.0) };
        w.objects.push(Box::new(Sphere { center: Point::new(0.0, -25.0, 0.0), radius: 80.0, material: mat1 }));
        w.objects.push(Box::new(Sphere { center: Point::new(0.0, 30.0, 0.0), radius: 60.0, material: mat2 }));
        w.objects.push(Box::new(
            Plane {
                point: Point::new(0.0, 10.0, 0.0),
                normal: Vec3::new(0.0, 0.0, -1.0),
                material: mat3,
            }
        ));
        w
    }

    pub fn move_in_z(&mut self, step: f32) {
        self.view_plane.z += step;
    }

    pub fn change_pixel_size(&mut self, step: f32) {
        self.view_plane.pixel_size += step;
        println!("pix size is now {}", self.view_plane.pixel_size);
    }

    pub fn change_resolution(&mut self, factor: f32) {
        self.view_plane.vres = (self.view_plane.vres as f32 * factor) as u32;
        self.view_plane.hres = (self.view_plane.hres as f32 * factor) as u32;
    }

    pub fn get_resolution(&self) -> (u32, u32) {
        (self.view_plane.hres, self.view_plane.vres)
    }
}