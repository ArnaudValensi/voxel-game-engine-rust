#![feature(fn_traits)]

extern crate glutin;
#[macro_use]
extern crate gfx;

pub mod transform;
pub use self::transform::Transform;

pub mod lifecycle;
pub use self::lifecycle::{Lifecycle, LifecycleEvent};

pub mod input;
pub use self::input::Input;

pub mod events;
pub use self::events::Events;

pub mod renderer;
pub use self::renderer::Renderer;

pub mod mesh;
pub use self::mesh::{Mesh, Pipe, Vertex};

pub mod camera;
pub use self::camera::Camera;

pub type Resources = gfx_device_gl::Resources;
pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;
