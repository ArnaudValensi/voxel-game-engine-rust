use super::glutin::{Event, EventsLoop, KeyboardInput, VirtualKeyCode, WindowEvent};
use super::{Input, Renderer};

pub struct Events {
    events_loop: EventsLoop,
    running: bool,
}

#[allow(clippy::new_without_default)]
impl Events {
    pub fn new() -> Self {
        Self {
            events_loop: EventsLoop::new(),
            running: true,
        }
    }

    pub fn get_events_loop(&mut self) -> &mut EventsLoop {
        &mut self.events_loop
    }

    pub fn update(&mut self, renderer: &mut Renderer, input: &Input) {
        let events_loop = &mut self.events_loop;
        let running = &mut self.running;

        events_loop.poll_events(|event| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *running = false,
                    WindowEvent::Resized(size) => {
                        renderer.resize(size);
                    }
                    _ => (),
                }
            }
        });
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}
