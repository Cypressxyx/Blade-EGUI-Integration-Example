mod render_engine;
mod window_application;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);

    let mut application = window_application::WindowApplication::default();

    event_loop.run_app(&mut application);
}
