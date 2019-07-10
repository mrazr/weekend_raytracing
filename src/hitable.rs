use crate::ray::{Ray, Vec3, Point};
use cgmath::{InnerSpace, dot};

const EPS: f32 = 0.0001;

pub trait Hitable {
    fn hit(&self, r: &Ray) -> Option<f32>;
}

#[derive(Copy, Clone)]
pub struct Sphere {
   pub center: Point,
   pub radius: f32,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray) -> Option<f32> {
        let a = r.v.magnitude2();
        let b = 2.0 * dot(r.p - self.center, r.v);
        let c = (r.p - self.center).magnitude2() - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let disc_sqrt = discriminant.sqrt();
            let t = (-b + disc_sqrt) * 0.5 / a;
            if t > EPS {
                return Some(t);
            }

            let t = (-b - disc_sqrt) * 0.5 / a;
            if t > EPS {
                return Some(t);
            }
        }
        None
    }
}