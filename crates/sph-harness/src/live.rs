// run_live: window, audio device, simulation thread, and ring buffer.

use std::sync::Arc;

use sph_core::{SphBackend, SphSim};
use sph_synth::SphSynth;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use crate::render::WgpuView;

struct LiveApp<B: SphBackend> {
    sim: SphSim<B>,
    synth: SphSynth,
    view: Option<WgpuView>,
}

impl<B: SphBackend + Send + 'static> ApplicationHandler for LiveApp<B> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.view.is_some() {
            return;
        }

        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("sph-synth"))
                .expect("create window"),
        );
        self.view = Some(pollster::block_on(WgpuView::new(window.clone())));
        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        let Some(view) = self.view.as_mut() else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(new_size) => view.resize(new_size),
            WindowEvent::RedrawRequested => {
                self.sim.step(1.0 / 60.0);
                view.render(self.sim.snapshot());
                view.window().request_redraw();
            }
            _ => {}
        }
    }
}

fn run_live<B: SphBackend + Send + 'static>(sim: SphSim<B>, synth: SphSynth) {
    let event_loop = EventLoop::new().expect("create winit event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = LiveApp { sim, synth, view: None };
    event_loop.run_app(&mut app).expect("run winit event loop");
}
