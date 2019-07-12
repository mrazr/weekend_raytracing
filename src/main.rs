mod ray;
mod hitable;
mod world;
mod tracer;
mod material;
mod sampler;
use minifb::Window;
use crate::world::World;
use crate::tracer::{ Tracer, Caster };
use crate::sampler::{ Sampler, SimpleSampler, RegularSampler, JitterSampler, RandomSampler };

fn main() {
    // let n = 255 | ((255.0 * 0.5) as u32) << 24 | ((255.0 * 0.25) as u32) << 16 | ((255.0 * 0.125) as u32) << 8;
    // let b = 255 | (((255.0 * 0.5) as u32) << 24) | (((255.0 * 0.25) as u32) << 16) | (((255.0 * 0.125) as u32) << 8);
    // println!("n = {}, \tb = {}", n, b);
    let mut world = World::demo();
    let (hr, vr) = (world.view_plane.hres, world.view_plane.vres);
    let mut buffer: Vec<u32> = Vec::with_capacity((hr * vr) as usize);
    (0..(hr * vr)).into_iter().for_each(|_| buffer.push(0u32));
    let sampler = RandomSampler::new(25);
    let mut tracer = Tracer::Caster(Caster { sampler: Box::new(sampler) });
    tracer.trace(&world, &mut buffer);
    let mut window = Window::new("Ray", hr as usize, vr as usize, minifb::WindowOptions::default()).unwrap();
    while window.is_open() && !window.is_key_released(minifb::Key::Escape) {
        if window.is_key_released(minifb::Key::W) {
            world.move_in_z(-10.0);
            tracer.trace(&world, &mut buffer);
        } else if window.is_key_released(minifb::Key::S) {
            world.move_in_z(10.0);
            tracer.trace(&world, &mut buffer);
        } else if window.is_key_released(minifb::Key::A) {
            world.change_pixel_size(-0.5);
            tracer.trace(&world, &mut buffer);
        } else if window.is_key_released(minifb::Key::D) {
            world.change_pixel_size(0.5);
            tracer.trace(&world, &mut buffer);
        } else if window.is_key_released(minifb::Key::Up) {
            world.change_resolution(2.0);
            recreate_buffer(&mut buffer, world.get_resolution());
            tracer.trace(&world, &mut buffer);
            window = recreate_window(world.get_resolution());
        } else if window.is_key_released(minifb::Key::Down) {
            world.change_resolution(0.5);
            recreate_buffer(&mut buffer, world.get_resolution());
            tracer.trace(&world, &mut buffer);
            window = recreate_window(world.get_resolution());
        } else if window.is_key_released(minifb::Key::J) {
            tracer.set_sampler(JitterSampler::new(16));
            tracer.trace(&world, &mut buffer);
        } else if window.is_key_released(minifb::Key::R) {
            tracer.set_sampler(RandomSampler::new(16));
            tracer.trace(&world, &mut buffer);
        } else if window.is_key_released(minifb::Key::G) {
            tracer.set_sampler(RegularSampler { sampler_window_size: 4 });
            tracer.trace(&world, &mut buffer);
        } else if window.is_key_released(minifb::Key::P) {
            tracer.set_sampler(SimpleSampler);
            tracer.trace(&world, &mut buffer);
        } else if window.is_key_released(minifb::Key::NumPadPlus) {
            tracer.increase_sample_window_size(2);
            tracer.trace(&world, &mut buffer);
        } else if window.is_key_released(minifb::Key::NumPadMinus) {
            tracer.decrease_sample_window_size(2);
            tracer.trace(&world, &mut buffer);
        } else if window.is_key_released(minifb::Key::K) {
            println!("setting 1 sampls");
            match tracer {
                Tracer::Caster(Caster { ref mut sampler }) => sampler.set_num_samples(1),
            }
            tracer.trace(&world, &mut buffer);
        }
        window.update_with_buffer(&buffer);
    }
}

fn recreate_buffer(buff: &mut Vec<u32>, dim: (u32, u32)) {
    buff.resize((dim.0 * dim.1) as usize, 0u32);
    buff.iter_mut().map(|v| *v = 0u32);
}

fn recreate_window(dim: (u32, u32)) -> Window {
    Window::new("Ray", dim.0 as usize, dim.1 as usize, minifb::WindowOptions::default()).unwrap()
}
