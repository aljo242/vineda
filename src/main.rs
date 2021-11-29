#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::single_match)]

extern crate glfw;

use glfw::{Action, Context, Key};

const WINDOW_TITLE: &str = "Hello Window";



fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    // create a windoed mode window and its openGL context
    let (mut window, events) = glfw.create_window(
        300, 300, WINDOW_TITLE,
        glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // make the windows current context
    window.make_current();
    window.set_key_polling(true);

    // loop
    while !window.should_close() {
        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                },
                _ => {},
            }
        }
    }
}
