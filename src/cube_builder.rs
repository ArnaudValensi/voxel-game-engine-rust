use super::{VoxelMesh, Renderer, Transform, Vertex};
use cgmath::Vector3;

pub fn cube_mesh_builder(renderer: &mut Renderer, position: Vector3<f32>, color: [f32; 3]) -> VoxelMesh {
    let vertices: Vec<Vertex> = vec![
        // Top (0, 0, 1)
        Vertex::new([-1, -1, 1], color),
        Vertex::new([1, -1, 1], color),
        Vertex::new([1, 1, 1], color),
        Vertex::new([-1, 1, 1], color),
        // Bottom (0, 0, -1)
        Vertex::new([-1, 1, -1], color),
        Vertex::new([1, 1, -1], color),
        Vertex::new([1, -1, -1], color),
        Vertex::new([-1, -1, -1], color),
        // Right (1, 0, 0)
        Vertex::new([1, -1, -1], color),
        Vertex::new([1, 1, -1], color),
        Vertex::new([1, 1, 1], color),
        Vertex::new([1, -1, 1], color),
        // Left (-1, 0, 0)
        Vertex::new([-1, -1, 1], color),
        Vertex::new([-1, 1, 1], color),
        Vertex::new([-1, 1, -1], color),
        Vertex::new([-1, -1, -1], color),
        // Front (0, 1, 0)
        Vertex::new([1, 1, -1], color),
        Vertex::new([-1, 1, -1], color),
        Vertex::new([-1, 1, 1], color),
        Vertex::new([1, 1, 1], color),
        // Back (0, -1, 0)
        Vertex::new([1, -1, 1], color),
        Vertex::new([-1, -1, 1], color),
        Vertex::new([-1, -1, -1], color),
        Vertex::new([1, -1, -1], color),
    ];

    let indices: Vec<u16> = vec![
        0, 1, 2, 2, 3, 0, // Top
        4, 5, 6, 6, 7, 4, // Bottom
        8, 9, 10, 10, 11, 8, // Right
        12, 13, 14, 14, 15, 12, // Left
        16, 17, 18, 18, 19, 16, // Front
        20, 21, 22, 22, 23, 20, // Back
    ];

    let up = Vector3::unit_y();
    let forward = Vector3::unit_z();
    let model = Transform::new(position, up, forward).get_transform();

    VoxelMesh::new(renderer, &vertices, &indices, model)
}
