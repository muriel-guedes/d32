use winit::{event_loop::{EventLoop, ControlFlow}, platform::run_return::EventLoopExtRunReturn,
    event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState}};

use crate::{Context, RasterShader, CopyShader};

pub struct Engine {
    pub event_loop: Option<EventLoop<()>>,
    pub context: Context
}
impl Engine {
    pub fn new() -> Self {
        crate::start_logger();
        
        let event_loop = EventLoop::new();
        let context = Context::new(&event_loop);

        Self {
            event_loop: Some(event_loop),
            context
        }
    }
    pub fn start(mut self) {
        log::trace!("Start");
        let c = self.context;
        self.event_loop.take().unwrap().run_return(|event, _, control_flow| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(key), state, .. }, .. } => {
                        match (key, state) {
                            (VirtualKeyCode::Escape, ElementState::Pressed) =>
                                *control_flow = ControlFlow::Exit,
                            _ => {}
                        }
                    },

                    WindowEvent::CursorLeft {..} => c.cursor.left(),
                    WindowEvent::CursorEntered {..} => c.cursor.entered(),
                    WindowEvent::Focused(focus) => if focus { c.cursor.entered() } else { c.cursor.left() },
                    WindowEvent::CursorMoved { position, .. } => c.cursor.moved(&c.window, position),
                    WindowEvent::MouseWheel { delta, .. } => match delta {
                        winit::event::MouseScrollDelta::LineDelta(x, y) => c.cursor.wheel_moved(x + y),
                        winit::event::MouseScrollDelta::PixelDelta(p) => c.cursor.wheel_moved((p.x + p.y) as f32)
                    },

                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(new_size) => c.resize(new_size),
                    _ => {}
                },
                Event::MainEventsCleared => c.window.request_redraw(),
                Event::RedrawRequested(_) => {
                    CopyShader::draw(&c);
                    c.update();
                    RasterShader::draw(&c);
                },
                _ => {}
            }
        });
    }
}