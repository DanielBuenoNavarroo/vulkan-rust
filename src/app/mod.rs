mod engine;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{WindowAttributes, WindowId},
};

use crate::app::engine::Engine;

#[derive(Default)]
pub struct App {
    engine: Option<Engine>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.engine = Some(Engine::new(event_loop).unwrap());
        // Create new window
        // if let Some(engine) = self.engine.as_mut() {
        //     let secondary_window = engine
        //         .create_window(
        //             event_loop,
        //             WindowAttributes::default().with_title("secondary_window"),
        //         )
        //         .unwrap();
        // }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(engine) = self.engine.as_mut() {
            engine.window_event(event_loop, window_id, event);
        }
    }
}
