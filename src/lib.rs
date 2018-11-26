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

pub mod pipeline;
pub use self::pipeline::Pipeline;

pub mod mesh;
pub use self::mesh::Mesh;

pub mod camera;
pub use self::camera::Camera;

pub mod cube_builder;
pub use self::cube_builder::cube_mesh_builder;

// pub mod gui;

pub type Resources = gfx_device_gl::Resources;
pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub mod voxel_mesh;
pub use self::voxel_mesh::{Vertex, VoxelMesh, VoxelMeshPipe};
