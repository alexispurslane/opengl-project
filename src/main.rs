extern crate egui;
extern crate egui_sdl2_gl;
extern crate gl;
extern crate glam;
extern crate rayon;
extern crate russimp;
extern crate sdl2;
#[macro_use]
extern crate project_gilgamesh_render_gl_derive as render_gl_derive;

use egui_backend::ShaderVersion;
use egui_sdl2_gl as egui_backend;
use entity::EntitySystem;
use scene::*;
use std::io::{stdout, Write};

mod entity;
mod events;
mod render_gl;
mod scene;
mod systems;
mod utils;

const NUM_INSTANCES: i32 = 100;
const UPDATE_INTERVAL: u128 = 16;

pub fn main() {
    ///////// Initialize SDL2 window

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_double_buffer(true);
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 3);

    let mut window = video_subsystem
        .window("Project Gilgamesh v0.1.0", 1920, 1080)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    sdl_context.mouse().set_relative_mouse_mode(true);

    ///////// Initialize OpenGL

    let _image_context = sdl2::image::init(sdl2::image::InitFlag::all());
    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    utils::setup_viewport(window.size());

    ///////// Initialize eGUI

    let shader_ver = ShaderVersion::Default;
    let (mut painter, mut egui_state) =
        egui_backend::with_sdl2(&window, shader_ver, egui_sdl2_gl::DpiScaling::Default);
    let mut egui_ctx = egui::CtxRef::default();

    ///////// Iniitalize game

    let mut scene = Scene {
        camera: None,
        command_queue: vec![],
        running: true,
        entities: EntitySystem::new(),
        shader_programs: vec![],
    };

    systems::add_camera(&mut scene);
    systems::add_level(&mut scene);
    systems::setup_mesh_components(&mut scene.entities);

    ///////// Game loop

    let start_time = std::time::Instant::now();
    let mut last_time = start_time.elapsed().as_millis();
    let mut dt;
    let mut lag = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();

    while scene.running {
        // Track time
        let time = start_time.elapsed().as_millis();

        dt = time - last_time;
        last_time = time;
        lag += dt;

        egui_state.input.time = Some(time as f64 / 1000.0f64);

        // Handle player input
        scene.queue_commands(events::handle_window_events(
            &mut window,
            &mut sdl_context.mouse(),
            &mut egui_state,
            &mut painter,
            event_pump.poll_iter(),
        ));
        scene.queue_commands(events::handle_keyboard(
            &scene,
            &event_pump.keyboard_state(),
            dt,
        ));
        scene.queue_commands(events::handle_mouse(
            &scene,
            &event_pump.relative_mouse_state(),
        ));
        scene.update(dt as f32);

        while lag >= UPDATE_INTERVAL {
            scene.queue_commands(systems::physics(&scene));
            scene.update(UPDATE_INTERVAL as f32);
            lag -= UPDATE_INTERVAL;
        }

        // Render ui
        egui_ctx.begin_frame(egui_state.input.take());

        egui::TopBottomPanel::bottom("bottom_panel").show(&egui_ctx, |ui| {
            ui.horizontal_top(|ui| {
                ui.label("Hello world");
            });
        });
        let (egui_output, paint_cmds) = egui_ctx.end_frame();
        egui_state.process_output(&window, &egui_output);
        let paint_jobs = egui_ctx.tessellate(paint_cmds);

        // Render world
        utils::clear_screen();
        let (width, height) = window.size();
        systems::render(&scene, width, height);

        painter.paint_jobs(None, paint_jobs, &egui_ctx.font_image());

        window.gl_swap_window();
    }
}
