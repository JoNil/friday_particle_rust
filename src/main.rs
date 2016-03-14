#[macro_use] extern crate glium;
extern crate cgmath;
extern crate rand;

use cgmath::*;
use glium::{Blend, BlendingFunction, LinearBlendingFactor, DisplayBuild, DrawParameters, Program, Surface, VertexBuffer};
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
struct VertexPos {
    pos: [f32; 2],
}

#[derive(Copy, Clone, Default)]
struct VertexTex {
    tex: [f32; 2],
}

implement_vertex!(VertexPos, pos);
implement_vertex!(VertexTex, tex);

fn build_vertex_pos_buffer(particles: &[Particle], vertices: &mut [VertexPos]) {

    assert!(particles.len() == vertices.len() / 6);

    for i in 0..particles.len() {

        vertices[i*6 + 0].pos[0] = -particles[i].size / 2.0 + particles[i].pos.x;
        vertices[i*6 + 0].pos[1] = -particles[i].size / 2.0 + particles[i].pos.y;

        vertices[i*6 + 1].pos[0] = particles[i].size / 2.0 + particles[i].pos.x;
        vertices[i*6 + 1].pos[1] = -particles[i].size / 2.0 + particles[i].pos.y;

        vertices[i*6 + 2].pos[0] = particles[i].size / 2.0 + particles[i].pos.x;
        vertices[i*6 + 2].pos[1] = particles[i].size / 2.0 + particles[i].pos.y;


        vertices[i*6 + 3].pos[0] = -particles[i].size / 2.0 + particles[i].pos.x;
        vertices[i*6 + 3].pos[1] = -particles[i].size / 2.0 + particles[i].pos.y;

        vertices[i*6 + 4].pos[0] = particles[i].size / 2.0 + particles[i].pos.x;
        vertices[i*6 + 4].pos[1] = particles[i].size / 2.0 + particles[i].pos.y;        

        vertices[i*6 + 5].pos[0] = -particles[i].size / 2.0 + particles[i].pos.x;
        vertices[i*6 + 5].pos[1] = particles[i].size / 2.0 + particles[i].pos.y;
    }
}

fn build_vertex_tex_buffer(vertices: &mut [VertexTex]) {

    for i in 0..(vertices.len() / 6) {

        vertices[i*6 + 0].tex[0] = 0.0;
        vertices[i*6 + 0].tex[1] = 0.0;

        vertices[i*6 + 1].tex[0] = 1.0;
        vertices[i*6 + 1].tex[1] = 0.0;

        vertices[i*6 + 2].tex[0] = 1.0;
        vertices[i*6 + 2].tex[1] = 1.0;


        vertices[i*6 + 3].tex[0] = 0.0;
        vertices[i*6 + 3].tex[1] = 0.0;

        vertices[i*6 + 4].tex[0] = 1.0;
        vertices[i*6 + 4].tex[1] = 1.0;

        vertices[i*6 + 5].tex[0] = 0.0;
        vertices[i*6 + 5].tex[1] = 1.0;
    }
}

const PARTICLE_COUNT: i32 = 1000;

fn main() {

    let display = glium::glutin::WindowBuilder::new()
        .with_title("Friday Particle".into())
        .with_dimensions(WIDTH as u32, HEIGHT as u32)
        .build_glium().unwrap();

    let program = Program::from_source(&display,"
        #version 120
        
        attribute vec2 pos;
        attribute vec2 tex;

        varying vec2 fragment_pos;
        varying vec2 fragment_tex;

        void main() {
            fragment_pos = pos;
            fragment_tex = tex;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    ", "
        #version 120
        
        varying vec2 fragment_pos;
        varying vec2 fragment_tex;

        void main()
        {
            vec3 color = vec3(191.0/255.0, 0.0, 1.0);

            vec2 local = (fragment_tex - 0.5) * 2.0;

            float r = sqrt(local.x*local.x + local.y*local.y);
            r = clamp(r, 0.0, 1.0);
            float alpha = 1.5;
            alpha*= pow(1.0 - r, 2.0);

            gl_FragColor = vec4(color, alpha);
        }
    ", None).unwrap();

    let mut particles: Vec<Particle> = vec![Default::default(); PARTICLE_COUNT as usize];

    let mut vertex_pos: Vec<VertexPos> = vec![Default::default(); 6 * PARTICLE_COUNT as usize];
    let vertex_pos_buffer: VertexBuffer<VertexPos> = VertexBuffer::dynamic(&display,
            &[Default::default(); 6 * PARTICLE_COUNT as usize]).unwrap();

    let vertex_tex_buffer: VertexBuffer<VertexTex> = {
        let mut vertex_tex: Vec<VertexTex> = vec![Default::default(); 6 * PARTICLE_COUNT as usize];
        build_vertex_tex_buffer(&mut vertex_tex);
        VertexBuffer::new(&display, &vertex_tex).unwrap()
    };

    let params = DrawParameters {
        blend: Blend {
            color: BlendingFunction::Addition { source: LinearBlendingFactor::SourceAlpha , destination: LinearBlendingFactor::OneMinusSourceAlpha },
            alpha: BlendingFunction::Addition { source: LinearBlendingFactor::SourceAlpha , destination: LinearBlendingFactor::OneMinusSourceAlpha },
            .. Default::default()
        },
        .. Default::default()
    };

    loop {

        simulate_particles(&mut particles, 1.0 / 60.0);

        build_vertex_pos_buffer(&particles, &mut vertex_pos);

        vertex_pos_buffer.write(&vertex_pos);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.2, 1.0);
        target.draw(
            (&vertex_pos_buffer, &vertex_tex_buffer),
            &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            &program,
            &glium::uniforms::EmptyUniforms,
            &params).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,   // the window has been closed by the user
                _ => ()
            }
        }
    }

}