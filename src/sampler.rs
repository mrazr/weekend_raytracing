use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::Rng;
use std::cell::RefCell;

pub trait Sampler {
    fn sample(&self, pixel_size: f32, pixel_center: (f32, f32)) -> Vec<(f32, f32)>; 
    fn num_samples(&self) -> u8;
    fn set_num_samples(&mut self, num: u8); 
}

pub struct SimpleSampler;

impl Sampler for SimpleSampler {
    fn sample(&self, _pixel_size: f32, pixel_center: (f32, f32)) -> Vec<(f32, f32)> {
        vec![pixel_center]
    }

    fn num_samples(&self) -> u8 {
        1
    }

    fn set_num_samples(&mut self, num: u8) {
        ()
    }
}

pub struct RegularSampler {
    pub sampler_window_size: u8,
}

impl Sampler for RegularSampler {
    fn sample(&self, pixel_size: f32, pixel_center: (f32, f32)) -> Vec<(f32, f32)> {
        let mut sample_coords = Vec::<(f32, f32)>::with_capacity((self.sampler_window_size * self.sampler_window_size) as usize);
        let step = pixel_size / self.sampler_window_size as f32;
        for i in 0..self.sampler_window_size {
            for j in 0..self.sampler_window_size {
                sample_coords.push(
                    (
                        pixel_center.0 - pixel_size * 0.5 + (j as f32 + 0.5) * step,
                        pixel_center.1 - pixel_size * 0.5 + (i as f32 + 0.5) * step
                    )
                );
            }
        }
        sample_coords
    }

    fn num_samples(&self) -> u8 {
        self.sampler_window_size * self.sampler_window_size
    }

    fn set_num_samples(&mut self, num: u8) {
        self.sampler_window_size = (num as f32).sqrt() as u8;
    }
}


pub struct JitterSampler {
    pub sampler_window_size: u8,
    rng: RefCell<SmallRng>, //using interior mutability, but should probably solve it without this, there is probably some overhead because of the dynamic borrow checking
}

impl JitterSampler {
    pub fn new(num_samples: u8) -> JitterSampler {
        JitterSampler {
            sampler_window_size: (num_samples as f32).sqrt() as u8,
            rng: RefCell::new(SmallRng::seed_from_u64(42u64)), // for now fix the seed to the totally random number 42
        }
    }
}

impl Sampler for JitterSampler {
    fn sample(&self, pixel_size: f32, pixel_center: (f32, f32)) -> Vec<(f32, f32)> {
        let mut sample_coords = Vec::<(f32, f32)>::with_capacity((self.sampler_window_size * self.sampler_window_size) as usize);
        let step = pixel_size / self.sampler_window_size as f32;
        let mut rng = self.rng.borrow_mut();
        for i in 0..self.sampler_window_size {
            for j in 0..self.sampler_window_size {
                sample_coords.push(
                    (
                        pixel_center.0 - pixel_size * 0.5 + (j as f32 + rng.gen::<f32>()) * step,
                        pixel_center.1 - pixel_size * 0.5 + (i as f32 + rng.gen::<f32>()) * step
                    )
                );
            }
        }
        sample_coords
    }

    fn num_samples(&self) -> u8 {
        self.sampler_window_size * self.sampler_window_size
    }

    fn set_num_samples(&mut self, num: u8) {
        self.sampler_window_size = (num as f32).sqrt() as u8;
    }
}

pub struct RandomSampler {
    pub sampler_window_size: u8,
    rng: RefCell<SmallRng>,
}

impl RandomSampler {
    pub fn new(num_samples: u8) -> RandomSampler {
        RandomSampler {
            sampler_window_size: (num_samples as f32).sqrt() as u8,
            rng: RefCell::new(SmallRng::seed_from_u64(42u64)),
        }
    }
}

impl Sampler for RandomSampler {
    fn sample(&self, pixel_size: f32, pixel_center: (f32, f32)) -> Vec<(f32, f32)> {
        let mut rng = self.rng.borrow_mut();
        (0..self.num_samples()).into_iter()
            .map(|_| (pixel_center.0 + rng.gen_range(-0.5, 0.5) * pixel_size, pixel_center.1 + rng.gen_range(-0.5, 0.5) * pixel_size))
            .collect()
    }

    fn num_samples(&self) -> u8 {
        self.sampler_window_size * self.sampler_window_size
    }

    fn set_num_samples(&mut self, num: u8) {
        self.sampler_window_size = (num as f32).sqrt() as u8;
    }
}