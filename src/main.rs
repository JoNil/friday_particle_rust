#[macro_use] extern crate glium;
extern crate cgmath;
extern crate rand;
extern crate time;

use cgmath::*;
use glium::glutin::{Api, GlProfile, GlRequest};
use glium::program::ProgramCreationError;
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
            pos: Vector2 { x: 0.0, y: Range::new(0.0, 5.0).ind_sample(&mut rand::thread_rng()) },
            speed: Vector2 { x: 0.0, y: 0.0 },
            acc: Vector2 { x: 0.0, y: 0.0 },
            size: 0.1,
        }
    }
}

fn simulate_particles(particles: &mut [Particle], dt: f32)
{
    for particle in particles {
        if -1.2 < particle.pos.y && particle.pos.y < -1.0 && -0.05 < particle.pos.x && particle.pos.x < 0.05 {
            particle.speed.y = Range::new(0.5, 2.0).ind_sample(&mut rand::thread_rng());
            particle.speed.x = Range::new(-1.5/5.0, 1.5/5.0).ind_sample(&mut rand::thread_rng());
            particle.acc.x = 0.0;
            particle.acc.y = 0.0;
        } else {
            particle.acc.y = -1.1;
            if particle.pos.y < -1.0 {
                particle.speed.y = -(particle.pos.y + 1.0) * 20.0;
                particle.acc.y = 0.0;
                particle.acc.x = -particle.pos.x * 20.0;
                if particle.pos.x < -0.02 && 0.02 < particle.pos.x {
                    particle.speed.x = 0.0;
                    particle.acc.x=0.0;
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
        .with_gl_profile(GlProfile::Core)
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_glium().unwrap();

    let program = match Program::from_source(&display,"
        #version 330 core
        
        layout(location = 0) in vec2 pos;
        layout(location = 1) in vec2 tex;

        out vec2 fragment_pos;
        out vec2 fragment_tex;

        void main() {
            fragment_pos = pos;
            fragment_tex = tex;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    ", "
        #version 330 core
        
        in vec2 fragment_pos;
        in vec2 fragment_tex;

        layout(location = 0) out vec4 fragment_color;

        uniform vec2 mouse_pos;

        void main()
        {
            vec2 local = (fragment_tex - 0.5) * 2.0;

            float shade = clamp(dot(local, normalize(mouse_pos)), 0.2, 1.0);
            float shade2 = clamp(dot(local, normalize(vec2(mouse_pos.y, -mouse_pos.x))), 0.2, 1.0);

            vec3 color = shade * vec3(191.0/255.0, 0.2, 1.0) + 
                         shade2 * vec3(0.9, 0.2, 0.1);

            float r = sqrt(local.x*local.x + local.y*local.y);
            r = clamp(r, 0.0, 1.0);
            float alpha = 1.5;
            alpha *= pow(1.0 - r, 2.0);

            fragment_color = vec4(color, alpha);
        }
    ", None) {
        Ok(prg) => prg,
        Err(err) => {
            if let ProgramCreationError::CompilationError(log) = err {
                println!("{}", log);
                panic!("Failed to compile shader");
            } else {
                panic!("{:?}", err)
            }
        }
    };

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

    let mut old_time = time::precise_time_ns();
    let mut mouse_pos = (0, 0);

    loop {

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::MouseMoved(pos) => mouse_pos = pos,
                _ => ()
            }
        }

        let new_time = time::precise_time_ns();
        let dt = (new_time - old_time) as f32 / 1e9;
        old_time = new_time;

        simulate_particles(&mut particles, dt);

        build_vertex_pos_buffer(&particles, &mut vertex_pos);

        vertex_pos_buffer.write(&vertex_pos);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.2, 1.0);
        target.draw(
            (&vertex_pos_buffer, &vertex_tex_buffer),
            &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            &program,
            &uniform! { mouse_pos: (
                (mouse_pos.0 as f32 / WIDTH as f32) * 2.0 - 1.0,
                (1.0 - mouse_pos.1 as f32 / HEIGHT as f32) * 2.0 - 1.0) },
            &params).unwrap();
        target.finish().unwrap();
    }

}