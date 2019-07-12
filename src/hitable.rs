use crate::ray::{Ray, Vec3, Point};
use cgmath::{InnerSpace, dot};
use crate::material::Material;

const EPS: f32 = 0.0001;

pub trait Hitable {
    fn hit(&self, r: &Ray) -> Option<f32>;
    fn color(&self) -> Vec3;
}

#[derive(Copy, Clone)]
pub struct Sphere {
   pub center: Point,
   pub radius: f32,
   pub material: Material,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray) -> Option<f32> {
        let a = r.v.magnitude2();
        let b = 2.0 * dot(r.p - self.center, r.v);
        let c = (r.p - self.center).magnitude2() - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let disc_sqrt = discriminant.sqrt();
            let mut t = (-b - disc_sqrt) * 0.5 / a;
            
            if t > EPS {
                return Some(t);
            }

            t = (-b + disc_sqrt) * 0.5 / a;

            if t > EPS {
                return Some(t);
            }
        }
        None
    }

    fn color(&self) -> Vec3 {
        self.material.color
    }
}

#[derive(Copy, Clone)]
pub struct Plane {
    pub point: Point,
    pub normal: Vec3,
    pub material: Material,
}

impl Hitable for Plane {
    fn hit(&self, r: &Ray) -> Option<f32> {
        // (a - p) * normal -> a is the ray's origin, p is the plane's point
        let ampn = dot(self.point - r.p, self.normal);
        if ampn.abs() < EPS {
            return None;
        }

        let t = ampn / dot(self.normal, r.v);
        if t < EPS {
            return None;
        }
        Some(t)
    }

    fn color(&self) -> Vec3 {
        self.material.color
    }
}


pub struct HitRecord<'a> {
    pub hit_point: Point,
    pub normal: Vec3,
    pub hit_obj: Option<&'a Box<dyn Hitable>>,
}