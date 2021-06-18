use rust_game_library_wgpu::objects::vertex::{Vertex};
use rust_game_library_wgpu::objects::tex_coord::TexCoord;
use nalgebra_glm::TVec3;
use crate::vertex::chunk_vertex::ChunkVertex;

#[derive(Clone)]
pub struct BlockSides {
    pub north: Vec<ChunkVertex>,
    pub east: Vec<ChunkVertex>,
    pub south: Vec<ChunkVertex>,
    pub west: Vec<ChunkVertex>,
    pub down: Vec<ChunkVertex>,
    pub up: Vec<ChunkVertex>
}

impl BlockSides {
    pub fn new() -> BlockSides {

        let west = vec![ChunkVertex::new(-0.5,0.5,-1.0,0.0,1.0,-1.0,0.0,0.0,0.4),ChunkVertex::new(-0.5,-0.5,-1.0,0.0,0.0,-1.0,0.0,0.0,0.4),ChunkVertex::new(-0.5,-0.5,0.0,1.0,0.0,-1.0,0.0,0.0,0.4),ChunkVertex::new(-0.5,0.5,0.0,1.0,1.0,-1.0,0.0,0.0,0.4)];

        let south = vec![ChunkVertex::new(-0.5,0.5,0.0,0.0,1.0,0.0,0.0,-1.0,0.6),ChunkVertex::new(-0.5,-0.5,0.0,0.0,0.0,0.0,0.0,-1.0,0.6),ChunkVertex::new(0.5,-0.5,0.0,1.0,0.0,0.0,0.0,-1.0,0.6),ChunkVertex::new(0.5,0.5,0.0,1.0,1.0,0.0,0.0,-1.0,0.6)];

        let east = vec![ChunkVertex::new(0.5,0.5,0.0,0.0,1.0,1.0,0.0,0.0,0.4),ChunkVertex::new(0.5,-0.5,0.0,0.0,0.0,1.0,0.0,0.0,0.4),ChunkVertex::new(0.5,-0.5,-1.0,1.0,0.0,1.0,0.0,0.0,0.4),ChunkVertex::new(0.5,0.5,-1.0,1.0,1.0,1.0,0.0,0.0,0.4)];

        let north = vec![ChunkVertex::new(0.5,0.5,-1.0,0.0,1.0,0.0,0.0,1.0,0.6),ChunkVertex::new(0.5,-0.5,-1.0,0.0,0.0,0.0,0.0,1.0,0.6),ChunkVertex::new(-0.5,-0.5,-1.0,1.0,0.0,0.0,0.0,1.0,0.6),ChunkVertex::new(-0.5,0.5,-1.0,1.0,1.0,0.0,0.0,1.0,0.6)];

        let up = vec![ChunkVertex::new(-0.5,0.5,-1.0,0.0,1.0,0.0,1.0,0.0,1.0),ChunkVertex::new(-0.5,0.5,0.0,0.0,0.0,0.0,1.0,0.0,1.0),ChunkVertex::new(0.5,0.5,0.0,1.0,0.0,0.0,1.0,0.0,1.0),ChunkVertex::new(0.5,0.5,-1.0,1.0,1.0,0.0,1.0,0.0,1.0)];

        let down = vec![ChunkVertex::new(-0.5,-0.5,-1.0,0.0,1.0,0.0,-1.0,0.0,0.2),ChunkVertex::new(-0.5,-0.5,0.0,0.0,0.0,0.0,-1.0,0.0,0.2),ChunkVertex::new(0.5,-0.5,0.0,1.0,0.0,0.0,-1.0,0.0,0.2),ChunkVertex::new(0.5,-0.5,-1.0,1.0,1.0,0.0,-1.0,0.0,0.2)];

        return BlockSides {
            north,
            east,
            south,
            west,
            down,
            up
        }
    }

    pub fn add(x: f32, y:f32, z:f32,mut vec: Vec<ChunkVertex>, tex_coords: &TexCoord) -> Vec<ChunkVertex> {
        for (id,v) in &mut vec.iter_mut().enumerate() {
            v.x += x;
            v.y += y;
            v.z += z;
            v.u = tex_coords.tex_coords[id].x;
            v.v = tex_coords.tex_coords[id].y;
        }

        return vec;
    }
}