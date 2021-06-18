use rust_game_library_wgpu::objects::vertex_buffer::OnlyCoordsVertexBuffer;

pub struct LineMesh {
    pub camera_offset: u32,
    pub mesh: OnlyCoordsVertexBuffer,
    pub should_render: bool
}

impl LineMesh {
    pub fn new(mesh: OnlyCoordsVertexBuffer) -> LineMesh {
        return LineMesh { camera_offset: 0, mesh, should_render: false };
    }
}