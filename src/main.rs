#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::single_match)]

extern crate glfw;
use self::glfw::{Action, Context, Key};

extern crate gl;

use std::sync::mpsc::Receiver;

const WINDOW_TITLE: &str = "Hello Window";
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;


fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4,6));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // create a windoed mode window and its openGL context
    let (mut window, events) = glfw.create_window(
        SCR_WIDTH, SCR_HEIGHT, WINDOW_TITLE,
        glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // make the windows current context
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // load all gl function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // loop
    while !window.should_close() {
        // EVENTS
        process_events(&mut window, &events);

        // RENDER
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // SWAP BUFFERS
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
   for (_, event) in glfw::flush_messages(events) {
    match event {
        glfw::WindowEvent::FramebufferSize(width, height) => {
            // make sure the viewport matches the new window dims
            unsafe { gl::Viewport(0, 0, width, height) }
        }
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
   } 
}
