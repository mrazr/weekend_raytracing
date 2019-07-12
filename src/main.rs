mod ray;
mod hitable;
mod world;
mod tracer;
mod material;
use minifb::Window;
use crate::world::World;
use crate::tracer::{ Tracer, Caster };

fn main() {
    // let n = 255 | ((255.0 * 0.5) as u32) << 24 | ((255.0 * 0.25) as u32) << 16 | ((255.0 * 0.125) as u32) << 8;
    // let b = 255 | (((255.0 * 0.5) as u32) << 24) | (((255.0 * 0.25) as u32) << 16) | (((255.0 * 0.125) as u32) << 8);
    // println!("n = {}, \tb = {}", n, b);
    let mut world = World::demo();
    let (hr, vr) = (world.view_plane.hres, world.view_plane.vres);
    let mut buffer: Vec<u32> = Vec::with_capacity((hr * vr) as usize);
    (0..(hr * vr)).into_iter().for_each(|_| buffer.push(0u32));
    {
        let tracer = Tracer::Caster(Caster { world: &world });
        tracer.trace(&mut buffer);
    }
    let mut window = Window::new("Ray", hr as usize, vr as usize, minifb::WindowOptions::default()).unwrap();
    while window.is_open() && !window.is_key_released(minifb::Key::Escape) {
        if window.is_key_released(minifb::Key::W) {
            world.move_in_z(-10.0);
            let tracer = Tracer::Caster(Caster { world: &world });
            tracer.trace(&mut buffer);
        } else if window.is_key_released(minifb::Key::S) {
            world.move_in_z(10.0);
            let tracer = Tracer::Caster(Caster { world: &world });
            tracer.trace(&mut buffer);
        }
        window.update_with_buffer(&buffer);
    }
}
