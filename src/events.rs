use super::glutin::{
    ElementState, Event, EventsLoop, KeyboardInput, MouseButton, MouseScrollDelta, TouchPhase,
    VirtualKeyCode, WindowEvent,
};
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

    pub fn update(&mut self, renderer: &mut Renderer, input_obj: &mut Input) {
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
                    WindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput {
                            state,
                            virtual_keycode,
                            ..
                        } => {
                            if let Some(key) = virtual_keycode {
                                match state {
                                    ElementState::Pressed => {
                                        input_obj.set_key_down(key);
                                    }
                                    ElementState::Released => {
                                        input_obj.set_key_up(key);
                                    }
                                }
                            }
                        }
                    },
                    WindowEvent::CursorMoved { position, .. } => {
                        let logical_size = renderer.window.get_inner_size().unwrap();
                        let window_center_x = logical_size.width / 2.0;
                        let window_center_y = logical_size.height / 2.0;

                        input_obj.set_mouse_position(
                            (position.x, position.y),
                            (window_center_x, window_center_y),
                        );
                    }
                    WindowEvent::MouseInput { state, button, .. } => match button {
                        MouseButton::Left => {
                            input_obj.set_mouse_left(state == ElementState::Pressed)
                        }
                        MouseButton::Right => {
                            input_obj.set_mouse_right(state == ElementState::Pressed)
                        }
                        MouseButton::Middle => {
                            input_obj.set_mouse_middle(state == ElementState::Pressed)
                        }
                        _ => {}
                    },
                    WindowEvent::MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_, y),
                        phase: TouchPhase::Moved,
                        ..
                    } => input_obj.set_mouse_wheel(y),
                    WindowEvent::MouseWheel {
                        delta: MouseScrollDelta::PixelDelta(logical_position),
                        phase: TouchPhase::Moved,
                        ..
                    } => input_obj.set_mouse_wheel(logical_position.y as f32),
                    _ => (),
                }
            }
        });
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}
