extern crate gl;
extern crate sdl2;
#[macro_use]
extern crate project_gilgamesh_render_gl_derive as render_gl_derive;

use sdl2::event::Event;
use std::ffi::CString;

mod render_gl;
mod utils;
use render_gl::data::OpaqueColorVertex;
use render_gl::shaders;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Project Gilgamesh v0.1.0", 1024, 768)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let vert_shader = shaders::Shader::from_source(
        &CString::new(include_str!("triangle.vert")).unwrap(),
        gl::VERTEX_SHADER,
    )
    .unwrap();

    let frag_shader = shaders::Shader::from_source(
        &CString::new(include_str!("triangle.frag")).unwrap(),
        gl::FRAGMENT_SHADER,
    )
    .unwrap();

    let shader_program = shaders::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    let vbo = shaders::VertexBufferObject::new_with_vec(
        gl::ARRAY_BUFFER,
        vec![
            OpaqueColorVertex {
                pos: (0.5, -0.5, 0.0).into(),
                clr: (1.0, 0.0, 0.0).into(),
            },
            OpaqueColorVertex {
                pos: (-0.5, -0.5, 0.0).into(),
                clr: (0.0, 1.0, 0.0).into(),
            },
            OpaqueColorVertex {
                pos: (-0.0, 0.5, 0.0).into(),
                clr: (0.0, 0.0, 1.0).into(),
            },
        ],
    );

    let vao = shaders::VertexArrayObject::new();
    vao.bind();
    vbo.bind();
    vbo.setup_vertex_attrib_pointers();
    vbo.unbind();
    vao.unbind();

    unsafe {
        gl::Viewport(0, 0, 1024, 768);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let start_time = std::time::Instant::now();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.set_used();
        shader_program.set_uniform_3f(
            &CString::new("GlobalColor").unwrap(),
            start_time.elapsed().as_secs_f32().sin(),
            0.0,
            start_time.elapsed().as_secs_f32().sin(),
        );

        vao.bind();
        vao.draw_arrays(gl::TRIANGLES, 0, 3);
        vao.unbind();

        window.gl_swap_window();
    }
}
