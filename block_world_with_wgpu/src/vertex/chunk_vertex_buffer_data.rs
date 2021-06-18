use rust_game_library_wgpu::objects::vertex::{Vertex, NormalVertex};
use crate::vertex::chunk_vertex::ChunkVertex;

pub struct ChunkVertexBufferData {
    pub vertecies: Vec<ChunkVertex>,
    pub indecies: Vec<u16>
}

impl ChunkVertexBufferData {
    pub fn new(vertecies: Vec<ChunkVertex>, indecies: Vec<u16>) -> ChunkVertexBufferData {
        return ChunkVertexBufferData {
            vertecies,
            indecies
        }
    }
}