// Splitting back into a workspace, once physics.wgsl is real SPH:
//   src/particles.rs -> crates/sph-core (Particle/ParticleSet, SoA fields as needed)
//   src/gpu_sim.rs    -> crates/sph-core-gpu (GpuBackend: buffers, bind groups, dispatch)
//   src/physics.wgsl  -> crates/sph-core-gpu/src/shaders/ (density.wgsl + forces.wgsl)
//   src/renderer.rs   -> crates/sph-harness (window + wgpu view already live there)
//   src/main.rs       -> crates/sph-harness/src/live.rs + main.rs (App becomes run_live's LiveApp)
// crates/ already has real dependencies wired and a working window+wgpu backend
// under sph-harness/src/render/ (untouched, just not referenced by the root
// Cargo.toml right now) — port gpu_sim.rs/physics.wgsl into sph-core-gpu's
// backend.rs/shaders, then swap this crate's `mod`s for path deps on crates/*.

mod gpu_sim;
mod particles;
mod renderer;

use std::sync::Arc;

use gpu_sim::GpuSim;
use renderer::Renderer;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

const DT: f32 = 1.0 / 60.0;

// The sim domain (physics.wgsl's walls/floor/ceiling) is a square in clip space
// with no aspect-ratio correction in renderer.wgsl, so the window must be square
// too or particles render stretched.
const WINDOW_SIZE: u32 = 800;

struct App {
    renderer: Option<Renderer>,
    gpu_sim: Option<GpuSim>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.renderer.is_some() {
            return;
        }

        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("sph-synth")
                        .with_inner_size(LogicalSize::new(WINDOW_SIZE, WINDOW_SIZE)),
                )
                .expect("create window"),
        );
        let renderer = pollster::block_on(Renderer::new(window.clone()));
        let gpu_sim = GpuSim::new(
            renderer.device().clone(),
            renderer.queue().clone(),
            &particles::initial_grid(48),
        );

        self.renderer = Some(renderer);
        self.gpu_sim = Some(gpu_sim);
        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        let (Some(renderer), Some(gpu_sim)) = (self.renderer.as_mut(), self.gpu_sim.as_mut()) else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(new_size) => renderer.resize(new_size),
            WindowEvent::RedrawRequested => {
                gpu_sim.step(DT);
                renderer.render(gpu_sim.particle_buffer(), gpu_sim.particle_count());
                renderer.window().request_redraw();
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("create winit event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App {
        renderer: None,
        gpu_sim: None,
    };
    event_loop.run_app(&mut app).expect("run winit event loop");
}
