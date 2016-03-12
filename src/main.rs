#[macro_use] extern crate glium;
#[macro_use] extern crate lazy_static;
extern crate cgmath;
extern crate rand;

use cgmath::*;
use glium::{DisplayBuild, Surface};
use rand::distributions::{IndependentSample, Range};

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 720;

#[derive(Copy, Clone, Debug)]
struct Particle {
    pos: Vector2<f32>,
    speed: Vector2<f32>,
    acc: Vector2<f32>,
    size: f32,
}

impl Default for Particle {
    fn default() -> Particle {
        Particle {
            pos: Vector2 { x: 0.0, y: -0.5 },
            speed: Vector2 { x: 0.0, y: 0.0 },
            acc: Vector2 { x: 0.0, y: 0.0 },
            size: 0.1,
        }
    }
}

fn simulate_particles(particles: &mut [Particle], dt: f32)
{
    let mut rng = rand::weak_rng();

    for particle in particles {
        if -0.55 < particle.pos.y && particle.pos.y < -0.45 && -0.05 < particle.pos.x && particle.pos.x < 0.05 {
            particle.speed.y = Range::new(0.0, 2.0).ind_sample(&mut rng);
            particle.speed.x = Range::new(-2.0/3.0, 2.0/3.0).ind_sample(&mut rng);
            particle.acc.x = 0.0;
            particle.acc.y = 0.0;
        } else {
            particle.acc.y = -(particle.pos.y + 0.5) * 2.0;
            if particle.pos.y < -0.5 {
                particle.acc.y = -(particle.pos.y + 0.5) * 20.0;
                if particle.pos.x < -0.05 || 0.05 < particle.pos.x {
                    particle.acc.x = -particle.pos.x * 20.0;
                } else {
                    particle.speed.x = 0.0;
                }
            }
        }
        particle.speed = particle.speed + particle.acc * dt;
        particle.pos = particle.pos + particle.speed * dt;        
    }
}

fn main() {

    let display = glium::glutin::WindowBuilder::new()
        .with_title("Friday Particle".into())
        .with_dimensions(WIDTH as u32, HEIGHT as u32)
        .build_glium().unwrap();

    let particles: Vec<Particle> = vec![Default::default(); 1000];

    loop {

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.2, 1.0);
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,   // the window has been closed by the user
                _ => ()
            }
        }
    }

}