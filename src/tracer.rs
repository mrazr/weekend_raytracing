use crate::hitable::Hitable;
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
        let (vp, s, hr, vr) = {
            let world = self.world;
            let vp = world.view_plane;
            (vp, vp.pixel_size, vp.hres as usize, vp.vres as usize)
        };
        
        for j in 0..vr {
            for i in 0..hr {
                let x = s * (i as f32 - 0.5 * hr as f32 + 0.5);
                let y = s * (j as f32 - 0.5 * hr as f32 + 0.5);
                let ray = Ray::new(Point::new(x, y, 1.0), 
                    Vec3::new(0.0, 0.0, -1.0));
                let color = color_to_u32(&self.cast_ray(&ray));
                buff[j * hr + i] = color;
            }
        }
    }

    fn cast_ray(&self, r: &Ray) -> Vec3 {
        for obj in self.world.objects.iter() {
            if let Some(_) = obj.hit(r) {
                return Vec3::new(1.0, 0.0, 0.0);
            }
        }
        Vec3::zero()
    }
}

fn color_to_u32(c: &Vec3) -> u32 {
    (((255.0 * c.z) as u32) << 0) | (((255.0 * c.y) as u32) << 8) | (((255.0 * c.x) as u32) << 16)
}