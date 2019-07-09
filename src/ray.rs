use cgmath::{Vector3, Point3, Zero};
use std::default::Default;

pub type Vec3 = Vector3<f32>;
pub type Point = Point3<f32>;

#[derive(Copy, Clone)]
pub struct Ray {
    pub p: Point,
    pub v: Vec3,
}

impl Ray {
    pub fn new(origin: Point, dir: Vec3) -> Ray {
        Ray {
            p: origin,
            v: dir,
        }
    }

    pub fn t(&self, t: f32) -> Point {
        self.p + t * self.v
    }
}

impl Default for Ray {
    fn default() -> Ray {
        Ray {
            p: Point::new(0.0, 0.0, 0.0),
            v: Vec3::zero(),
        }
    }
}

pub fn zero_point() -> Point {
    Point::new(0.0, 0.0, 0.0)
}