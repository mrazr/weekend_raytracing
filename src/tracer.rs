use crate::hitable::{ Hitable, HitRecord };
use crate::world::{ World, ViewPlane };
use crate::ray::*;
use cgmath::Zero;

pub enum Tracer<'a> {
    Caster(Caster<'a>)
}

impl<'a> Tracer<'a> {
    pub fn trace(&self, buff: &mut [u32]) {
        match self {
            Tracer::Caster(caster) => caster.trace(buff),
        }
    } 
}

pub struct Caster<'a> {
    pub world: &'a World,
}

impl<'a> Caster<'a> {
    fn trace(&self, buff: &mut [u32]) {
        let (vp, s, hr, vr, z) = {
            let world = self.world;
            let vp = world.view_plane;
            (vp, vp.pixel_size, vp.hres as usize, vp.vres as usize, vp.z)
        };
        
        for j in 0..vr {
            for i in 0..hr {
                // let x = s * (i as f32 - 0.5 * hr as f32 + 0.5);
                // let y = s * (j as f32 - 0.5 * hr as f32 + 0.5);
                let x = s * (i as f32 - 0.5 * (hr as f32 - 1.0));
                let y = s * (j as f32 - 0.5 * (vr as f32  - 1.0));
                let ray = Ray::new(Point::new(x, y, z), 
                    Vec3::new(0.0, 0.0, -1.0));
                let color = color_to_u32(&self.cast_ray(&ray));
                buff[j * hr + i] = color;
            }
        }
    }

    fn cast_ray(&self, r: &Ray) -> Vec3 {
        let mut t_min = std::f32::MAX;
        let mut hit_rec = HitRecord { hit_point: zero_point(), normal: Vec3::zero(), hit_obj: None };
        for obj in self.world.objects.iter() {
            if let Some(t) = obj.hit(r) {
                if t < t_min {
                    t_min = t;
                    hit_rec.hit_obj = Some(obj);
                }
            }
        }
        if let Some(obj) = hit_rec.hit_obj {
            return obj.color();
        }
        self.world.background_color
    }
}

fn color_to_u32(c: &Vec3) -> u32 {
    (((255.0 * c.z) as u32) << 0) | (((255.0 * c.y) as u32) << 8) | (((255.0 * c.x) as u32) << 16)
}