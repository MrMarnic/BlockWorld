use std::collections::HashMap;
use nalgebra_glm::TVec2;
use std::rc::Rc;
use std::sync::Arc;
use crate::block::texture_coords::TextureCoords;
use crate::vertex::chunk_vertex::ChunkVertex;
use crate::block::location::Direction;
use crate::block::block_sides::BlockSides;

pub struct BlockType {
    pub id: String,
    pub tex_coords: TextureCoords,
    pub is_air: bool,
    pub is_transparent: bool,
    pub is_default_model: bool
}

impl BlockType {
    pub fn get_indecies(&self,start:u16, indecies:&mut Vec<u16>) {
        indecies.push(start as u16);
        indecies.push(start as u16+1);
        indecies.push(start as u16+3);
        indecies.push(start as u16+3);
        indecies.push(start as u16+1);
        indecies.push(start as u16+2);
    }

    pub fn get_vertices(&self, vertices:&mut Vec<ChunkVertex>, direction:&Direction, x:f32, y:f32, z:f32, block_sides:Arc<BlockSides>) {
        match direction {
            &Direction::DOWN => {
                vertices.extend(BlockSides::add(x,y,z,block_sides.down.clone(),&self.tex_coords.down));
            },
            &Direction::UP => {
                vertices.extend(BlockSides::add(x,y,z,block_sides.up.clone(),&self.tex_coords.up));
            },
            &Direction::NORTH => {
                vertices.extend(BlockSides::add(x,y,z,block_sides.north.clone(),&self.tex_coords.north));
            },
            &Direction::SOUTH => {
                vertices.extend(BlockSides::add(x,y,z,block_sides.south.clone(),&self.tex_coords.south));
            },
            &Direction::EAST => {
                vertices.extend(BlockSides::add(x,y,z,block_sides.east.clone(),&self.tex_coords.east));
            },
            &Direction::WEST => {
                vertices.extend(BlockSides::add(x,y,z,block_sides.west.clone(),&self.tex_coords.west));
            },
        }
    }
}