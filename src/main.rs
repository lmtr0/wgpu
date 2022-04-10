extern crate glfw;

use glfw::{Action, Key, WindowEvent};

mod state;
use state::State;

pub async fn run() {
    env_logger::builder().default_format().filter(None, log::LevelFilter::Debug).init();
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "Test Application", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.set_size_polling(true);
    window.set_key_polling(true);

    let mut state = State::new(&window).await;

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut state, &mut window, event);
        }
        state.update();

        log::info!("Rendering...");
        match state.render() {
            Ok(_) => {}
            // Reconfigure the surface if lost
            Err(wgpu::SurfaceError::Lost) => state.resize(state.width, state.height),
            // The system is out of memory, we should probably quit
            Err(wgpu::SurfaceError::OutOfMemory) => window.set_should_close(true),
            // All other errors (Outdated, Timeout) should be resolved by the next frame
            Err(e) => eprintln!("{:?}", e),
        }
        log::info!("Done");

    }
}

fn main() {
    println!("Running....");
    pollster::block_on(run());
    println!("Finished running");
}

fn handle_window_event(state: &mut State, window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }

        WindowEvent::Size(width, height) => {
            state.resize(width, height);
        }

        WindowEvent::Key(key, code, action, mods) => {
            if !state.input(key, &code, &action, &mods) {
                window.set_should_close(true);
            }
        }

        _ => {}
    }
}