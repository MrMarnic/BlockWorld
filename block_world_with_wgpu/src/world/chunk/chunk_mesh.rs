use crate::world::world::World;
use std::sync::Arc;
use crate::world::chunk::chunk::Chunk;
use crate::block::block_sides::BlockSides;
use crate::block::block_type::BlockType;
use crate::world::chunk::chunk_handler::{ChunkHandler, ChunkGroup};
use crate::block::location::{BlockLocation, Direction};
use rust_game_library_wgpu::objects::vertex::{Vertex, NormalVertex};
use rust_game_library_wgpu::objects::vertex_buffer::{VertexBuffer, NormalVertexBuffer};
use wgpu::Device;
use rust_game_library_wgpu::objects::vertex_buffer_data::{VertexBufferData, NormalVertexBufferData};
use crate::world::chunk::render_chunk::RenderChunk;
use crate::vertex::chunk_vertex_buffer_data::ChunkVertexBufferData;
use crate::vertex::chunk_vertex::ChunkVertex;
use crate::vertex::chunk_vertex_buffer::ChunkVertexBuffer;

pub struct ChunkMesh {
    pub mesh: ChunkVertexBuffer
}

impl ChunkMesh {

    pub fn new_data(big_chunk:&Chunk, chunk: &RenderChunk, blocks_sides: Arc<BlockSides>, group:&ChunkGroup) -> ChunkVertexBufferData{
        let mut vertices : Vec<ChunkVertex> = Vec::new();
        let mut indecies : Vec<u16> = Vec::new();
        //let mut light_map: LightMap = LightMap::new(chunk);


        fn is_transparent_and_self(blocksZ: Arc<BlockType>, loc_test:&BlockLocation, chunk:&RenderChunk, big_chunk:&Chunk, normal_loc:&BlockLocation,group:&ChunkGroup) -> bool {
            if loc_test.is_outside_of_x_z(chunk) {
                return group.is_transparent_and_self(blocksZ.clone(),loc_test);
            }else {
                if !loc_test.is_outside_of_y(chunk) {
                    return chunk.is_transparent_and_self(blocksZ.clone(),&normal_loc)
                } else {
                    return big_chunk.is_transparent_and_self(blocksZ.clone(),&loc_test.get_chunk_values(&big_chunk.location));
                }
            }
        }

        for (y,blocksY) in chunk.blocks.iter().enumerate() {
            for (x,blocksX) in blocksY.iter().enumerate() {
                for (z,blocksZ) in blocksX.iter().enumerate() {
                    let loc = BlockLocation::new_from(x as i32,y as i32,z as i32);

                    let loc_test = BlockLocation::new_from(x as i32 + chunk.location.x as i32 * (crate::world::chunk::chunk::WIDTH as i32), y as i32 + chunk.location.y as i32 * (crate::world::chunk::render_chunk::HEIGHT as i32), z as i32 + chunk.location.z as i32 * (crate::world::chunk::chunk::DEPTH as i32));

                    if !blocksZ.is_air {
                        let pos_x = x as f32 + chunk.location.x as f32 * (crate::world::chunk::chunk::WIDTH as f32 - 1.0);
                        let pos_z = z as f32 + chunk.location.z as f32 * (crate::world::chunk::chunk::DEPTH as f32 - 1.0);
                        if blocksZ.is_default_model {
                            if is_transparent_and_self(blocksZ.clone(),&loc_test.down(),chunk,big_chunk,&loc.down(),group) {
                                blocksZ.get_indecies(vertices.len() as u16,&mut indecies);
                                blocksZ.get_vertices(&mut vertices, &Direction::DOWN,pos_x,y as f32,pos_z,blocks_sides.clone());
                                /*
                                let start = vertices.len();
                                indecies.push(start as u16);
                                indecies.push(start as u16+1);
                                indecies.push(start as u16+3);
                                indecies.push(start as u16+3);
                                indecies.push(start as u16+1);
                                indecies.push(start as u16+2);
                                vertices.extend(BlockSides::add(pos_x,y as f32,pos_z,blocks_sides.down.clone(),&blocksZ.tex_coords.down));
                                 */
                            }
                            if is_transparent_and_self(blocksZ.clone(),&loc_test.up(),chunk,big_chunk,&loc.up(),group) {
                                blocksZ.get_indecies(vertices.len() as u16,&mut indecies);
                                blocksZ.get_vertices(&mut vertices, &Direction::UP,pos_x,y as f32,pos_z,blocks_sides.clone());
                                /*
                                let start = vertices.len();
                                indecies.push(start as u16);
                                indecies.push(start as u16+1);
                                indecies.push(start as u16+3);
                                indecies.push(start as u16+3);
                                indecies.push(start as u16+1);
                                indecies.push(start as u16+2);
                                vertices.extend(BlockSides::add(pos_x,y as f32,pos_z,blocks_sides.up.clone(),&blocksZ.tex_coords.up));
                                 */
                            }
                            if is_transparent_and_self(blocksZ.clone(),&loc_test.east(),chunk,big_chunk,&loc.east(),group) {
                                blocksZ.get_indecies(vertices.len() as u16,&mut indecies);
                                blocksZ.get_vertices(&mut vertices, &Direction::EAST,pos_x,y as f32,pos_z,blocks_sides.clone());
                                /*
                                let start = vertices.len();
                                indecies.push(start as u16);
                                indecies.push(start as u16+1);
                                indecies.push(start as u16+3);
                                indecies.push(start as u16+3);
                                indecies.push(start as u16+1);
                                indecies.push(start as u16+2);
                                vertices.extend(BlockSides::add(pos_x,y as f32,pos_z,blocks_sides.east.clone(),&blocksZ.tex_coords.east));
                                 */
                            }
                            if is_transparent_and_self(blocksZ.clone(),&loc_test.south(),chunk,big_chunk,&loc.south(),group) {
                                /*
                                let start = vertices.len();
                                indecies.push(start as u16);
                                indecies.push(start as u16+1);
                                indecies.push(start as u16+3);
                                indecies.push(start as u16+3);
                                indecies.push(start as u16+1);
                                indecies.push(start as u16+2);
                                vertices.extend(BlockSides::add(pos_x,y as f32,pos_z,blocks_sides.south.clone(),&blocksZ.tex_coords.south));
                                 */
                                blocksZ.get_indecies(vertices.len() as u16,&mut indecies);
                                blocksZ.get_vertices(&mut vertices, &Direction::SOUTH,pos_x,y as f32,pos_z,blocks_sides.clone());
                            }
                            if is_transparent_and_self(blocksZ.clone(),&loc_test.north(),chunk,big_chunk,&loc.north(),group) {
                                /*
                                let start = vertices.len();
                                indecies.push(start as u16);
                                indecies.push(start as u16+1);
                                indecies.push(start as u16+3);
                                indecies.push(start as u16+3);
                                indecies.push(start as u16+1);
                                indecies.push(start as u16+2);
                                vertices.extend(BlockSides::add(pos_x,y as f32,pos_z,blocks_sides.north.clone(),&blocksZ.tex_coords.north));
                                 */
                                blocksZ.get_indecies(vertices.len() as u16,&mut indecies);
                                blocksZ.get_vertices(&mut vertices, &Direction::NORTH,pos_x,y as f32,pos_z,blocks_sides.clone());
                            }
                            if is_transparent_and_self(blocksZ.clone(),&loc_test.west(),chunk,big_chunk,&loc.west(),group) {
                                /*
                                let start = vertices.len();
                                indecies.push(start as u16);
                                indecies.push(start as u16+1);
                                indecies.push(start as u16+3);
                                indecies.push(start as u16+3);
                                indecies.push(start as u16+1);
                                indecies.push(start as u16+2);
                                vertices.extend(BlockSides::add(pos_x,y as f32,pos_z,blocks_sides.west.clone(),&blocksZ.tex_coords.west));
                                 */
                                blocksZ.get_indecies(vertices.len() as u16,&mut indecies);
                                blocksZ.get_vertices(&mut vertices, &Direction::WEST,pos_x,y as f32,pos_z,blocks_sides.clone());
                            }
                        } else {
                            blocksZ.get_indecies(vertices.len() as u16,&mut indecies);
                            blocksZ.get_vertices(&mut vertices, &Direction::UP,pos_x,y as f32,pos_z,blocks_sides.clone());
                        }
                    }

                }
            }
        }

        return ChunkVertexBufferData::new(vertices,indecies);
    }

}