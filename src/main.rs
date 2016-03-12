#[macro_use] extern crate glium;
#[macro_use] extern crate lazy_static;
extern crate cgmath;
extern crate rand;

use cgmath::*;
use glium::VertexBuffer;
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

#[derive(Copy, Clone, Default)]
struct Vertex {
    pos: [f32; 3],
    tex: [f32; 2],
}

implement_vertex!(Vertex, pos, tex);

fn build_vertex_buffer(particles: &[Particle], vertices: &mut [Vertex]) {

    assert!(particles.len() == vertices.len() / 4);

    for i in 0..particles.len() {

        vertices[i*4 + 0].pos[0] = -particles[i].size / 2.0 + particles[i].pos.x;
        vertices[i*4 + 0].pos[1] = -particles[i].size / 2.0 + particles[i].pos.y; 
        vertices[i*4 + 0].pos[2] = 0.0;

        vertices[i*4 + 1].pos[0] = particles[i].size / 2.0 + particles[i].pos.x;
        vertices[i*4 + 1].pos[1] = -particles[i].size / 2.0 + particles[i].pos.y;
        vertices[i*4 + 1].pos[2] = 0.0;

        vertices[i*4 + 2].pos[0] = particles[i].size / 2.0 + particles[i].pos.x;
        vertices[i*4 + 2].pos[1] = particles[i].size / 2.0 + particles[i].pos.y;
        vertices[i*4 + 2].pos[2] = 0.0;

        vertices[i*4 + 3].pos[0] = -particles[i].size / 2.0 + particles[i].pos.x;
        vertices[i*4 + 3].pos[1] = particles[i].size / 2.0 + particles[i].pos.y;
        vertices[i*4 + 3].pos[2] = 0.0;

        vertices[i*4 + 0].tex[0] = 0.0;
        vertices[i*4 + 0].tex[1] = 0.0;

        vertices[i*4 + 1].tex[0] = 1.0;
        vertices[i*4 + 1].tex[1] = 0.0;

        vertices[i*4 + 2].tex[0] = 1.0;
        vertices[i*4 + 2].tex[1] = 1.0;

        vertices[i*4 + 3].tex[0] = 0.0;
        vertices[i*4 + 3].tex[1] = 1.0;
    }
}

const PARTICLE_COUNT: i32 = 1000;

fn main() {

    let display = glium::glutin::WindowBuilder::new()
        .with_title("Friday Particle".into())
        .with_dimensions(WIDTH as u32, HEIGHT as u32)
        .build_glium().unwrap();

    let program = program!(&display,
        130 => {
            vertex: "
                #version 130
                
                attribute vec3 vertex_pos;
                attribute vec2 vertex_tex;

                varying vec3 pos;
                varying vec2 tex;

                void main() {
                    pos = vertex_pos;
                    tex = vertex_tex;
                    gl_Position = vec4(vertex_pos, 1.0);
                }
            ",

            fragment: "
                #version 130
                
                varying vec3 pos;
                varying vec2 tex;

                void main()
                {
                    vec3 color = vec3(191.0/255.0, 0.0, 1.0);

                    vec2 local = (tex - 0.5) * 2.0;

                    float r = sqrt(local.x*local.x + local.y*local.y);
                    r = clamp(r, 0.0, 1.0);
                    float alpha = 1.5;
                    alpha*= pow(1.0 - r, 2.0);

                    gl_FragColor = vec4(color, alpha);
                }
            ",
        },
    ).unwrap();

    let mut particles: Vec<Particle> = vec![Default::default(); PARTICLE_COUNT as usize];

    let mut vertices: Vec<Vertex> = vec![Default::default(); 4 * PARTICLE_COUNT as usize];
    let vertex_buffer: VertexBuffer<Vertex> = VertexBuffer::dynamic(&display, &[Default::default(); 4 * PARTICLE_COUNT as usize]).unwrap();

    loop {

        simulate_particles(&mut particles, 1.0 / 60.0);

        build_vertex_buffer(&particles, &mut vertices);

        vertex_buffer.write(&vertices);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.2, 1.0);
        target.draw(&vertex_buffer, &glium::index::NoIndices, &program, &glium::uniforms::EmptyUniforms,
            &Default::default()).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,   // the window has been closed by the user
                _ => ()
            }
        }
    }

}