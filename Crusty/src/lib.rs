#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

// Extern crates
extern crate glutin;
extern crate gl;
extern crate rand;

// Useages
use gl::types::*;
use rand::prelude::*;
use std::time::{Duration, Instant};

// Constants
const TITLE: &str = "Crusty";

pub struct Application {
    EventsLoop: glutin::EventsLoop,
    WindowedContext: glutin::WindowedContext<glutin::PossiblyCurrent>,
}

impl Application {
    pub fn New() -> Self {
        // Create an icon for the window
        let vec = vec![0,0,0,0];
        let icon = glutin::Icon::from_rgba(vec, 1, 1).unwrap();

        // Create an events loop to capture window events
        let mut eventsLoop = glutin::EventsLoop::new();

        // Create a window
        let wb = glutin::WindowBuilder::new()
            .with_title(TITLE)
            .with_window_icon(Some(icon))
            .with_decorations(true)
            .with_dimensions(glutin::dpi::LogicalSize::new(1920.0, 1080.0));

        // Create a windowed context that contains a rendercontext and a window/eventsloop
        let windowedContext = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Latest)
            .with_srgb(true)
            .build_windowed(wb, &eventsLoop)
            .unwrap();

        // Make the windowcontext the current one
        let windowedContext = unsafe { windowedContext.make_current().unwrap() };

        // Create the struct 
        return Application {
            EventsLoop: eventsLoop,
            WindowedContext: windowedContext,
        }
    }

    pub fn Run(mut self) {
        // Get the window instance from the windowedContext
        let window = self.WindowedContext.window();

        // Load opengl with the correct proces address
        let gl = gl::load_with(|s| self.WindowedContext.get_proc_address(s) as *const _);

        // Set opengl data before starting
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }

        // Run a loop for ever to capture events and draw things
        let mut running = true;
        let mut updateInterval = Instant::now();

        while running {
            // create a time instance to calculate the fps with
            let now = Instant::now();

            // Handle events to the window
            self.EventsLoop.poll_events(|event| {
                // Window events
                match event {
                    // Window events
                    glutin::Event::WindowEvent { event, .. } => {
                        match event {
                            glutin::WindowEvent::CloseRequested => running = false,
                            glutin::WindowEvent::MouseInput { state, button, .. } => unsafe {
                                if state == glutin::ElementState::Pressed && button == glutin::MouseButton::Left {
                                    gl::ClearColor(thread_rng().gen::<f32>(), thread_rng().gen::<f32>(), thread_rng().gen::<f32>(), 1.0)
                                }
                            }
                            _ => (),
                        }
                    }

                    // Device Events
                    glutin::Event::DeviceEvent { event, .. } => {
                        match event {
                            _ => (),
                        }
                    }

                    // The rest i.e. Awakened and suspended
                    _ => ()
                }
            });

            // Draw things here
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            self.WindowedContext.swap_buffers().unwrap();

            // Set the title of the window with the current fps of the engine
            if updateInterval.elapsed().as_millis() >= 250 {
                window.set_title(&format!("{} - FPS: {:.0}", TITLE, 1.0 / (now.elapsed().as_micros() as f64 / 1_000_000.0 as f64)));
                updateInterval = Instant::now();
            }
        }
    }
}
