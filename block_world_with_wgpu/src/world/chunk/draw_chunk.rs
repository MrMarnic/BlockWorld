use crate::world::chunk::chunk_mesh::ChunkMesh;
use crate::world::chunk::render_chunk::RenderChunkLocation;
use rust_game_library_wgpu::objects::camera::Camera;
use wgpu::Queue;
use nalgebra_glm::TMat4;
use rust_game_library_wgpu::engine::game_engine::GameEngine;

pub struct DrawChunk {
    pub camera_offset: u32,
    pub mesh: Option<ChunkMesh>,
    pub loc: RenderChunkLocation,
    pub world_pos: TMat4<f32>
}

impl DrawChunk {
    pub fn write(&self,camera:&Camera,queue:&Queue) {
        queue.write_buffer(&camera.buffers[2],self.camera_offset as u64,&*rust_game_library_wgpu::objects::matrix_helper::get_bytes(&self.world_pos));
    }

    pub fn new(loc:RenderChunkLocation,engine:&mut GameEngine) -> Self {
        Self {
            camera_offset: engine.static_offset_handler.get_offset() as u32,
            mesh: None,
            loc: loc.clone(),
            world_pos: nalgebra_glm::translation(&loc.vec3())
        }
    }
}