use winit::event;

#[derive(Default)]
pub struct WindowApplication {
    pub window: Option<winit::window::Window>,
}

impl winit::application::ApplicationHandler for WindowApplication {
    /*
     * Implements the resumed function.
     * This is called by the operation system.
     * it is normally called only once in desktop but could be called multipole times in
     * ios/android
     */
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes =
                winit::window::Window::default_attributes().with_title("Test Window Application");
            let window = event_loop.create_window(window_attributes).unwrap();
            self.window = Some(window)
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: event::WindowEvent,
    ) {
        match event {
            _ => (),
        }
    }
}
