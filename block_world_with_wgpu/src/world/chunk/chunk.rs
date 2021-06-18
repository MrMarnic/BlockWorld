use crate::block::block_type::BlockType;
use std::sync::{Arc, Mutex};
use crate::block::block_sides::BlockSides;
use nalgebra_glm::{TMat4, TVec3, vec3};
use crate::block::block_registry::BlockRegistry;
use std::path::Path;
use binary_rw::{BinaryReader, BinaryWriter};
use crate::world::chunk::chunk_mesh::ChunkMesh;
use crate::block::location::BlockLocation;
use crate::player::player::Player;
use rust_game_library_wgpu::objects::aabb::AABB;
use crate::data::save::Save;
use rust_game_library_wgpu::objects::camera::Camera;
use wgpu::Queue;
use crate::world::chunk::render_chunk::{RenderChunk, RenderChunkLocation};
use std::collections::HashMap;
use rust_game_library_wgpu::objects::vertex_buffer_data::{VertexBufferData, NormalVertexBufferData};
use crate::world::world::World;
use rand::{random, Rng};
use std::ops::Index;
use crate::world::chunk::chunk_handler::ChunkGroup;
use noise::{NoiseFn, OpenSimplex, Perlin};
use crate::vertex::chunk_vertex_buffer_data::ChunkVertexBufferData;
use crate::world::biome::biome_selector::BiomeSelector;
use crate::world::biome::biome_type::BiomeType;

pub static WIDTH : usize = 16;
pub static HEIGHT : usize = 16 * 6;
pub static DEPTH : usize = 16;

#[derive(Clone)]
pub struct Chunk {
    pub location: ChunkLocation,
    pub block_sides: Arc<BlockSides>,
    pub aabb: AABB,
    pub render_chunks: Vec<RenderChunk>,
    pub size: i32,
    pub biome_data: Vec<Vec<Arc<BiomeType>>>
}

#[derive(Clone,PartialEq, Eq, Hash)]
pub struct ChunkLocation {
    pub x: i32,
    pub z: i32
}

impl ChunkLocation {
    pub fn vec3(&self) -> TVec3<f32> {
        return vec3(self.x as f32,0.0, self.z as f32);
    }

    pub fn equals(&self,other:&ChunkLocation) -> bool {
        return self.x == other.x && self.z == other.z;
    }

    pub fn new(x:i32,z:i32) -> ChunkLocation {
        return ChunkLocation {
            x,
            z
        };
    }

    pub fn north(&self) -> ChunkLocation {
        return ChunkLocation::new(self.x,self.z+1);
    }

    pub fn east(&self) -> ChunkLocation {
        return ChunkLocation::new(self.x + 1,self.z);
    }

    pub fn south(&self) -> ChunkLocation {
        return ChunkLocation::new(self.x,self.z-1);
    }

    pub fn west(&self) -> ChunkLocation {
        return ChunkLocation::new(self.x - 1,self.z);
    }

    pub fn get_surrounding_chunks(&self) -> Vec<ChunkLocation> {
        return vec![self.north(),self.east(),self.south(),self.west()];
    }
}
impl Chunk {
    pub fn new(working_dir:String,location: ChunkLocation,block_registry:Arc<Mutex<BlockRegistry>>, block_sides: Arc<BlockSides>,noise:Arc<OpenSimplex>,selector:Arc<BiomeSelector>) -> Chunk {
        let mut render_chunks = vec![];

        let mut biome_types: Vec<Vec<Arc<BiomeType>>> = Vec::with_capacity(WIDTH);
        for _ in 0..WIDTH {
            biome_types.push(Vec::with_capacity(DEPTH));
        }

        let mut blocks : Vec<Vec<Vec<Arc<BlockType>>>> = Chunk::generate_blocks(block_registry.clone(),noise,&location,selector,&mut biome_types);

        for y in 0..6 {
            render_chunks.push(RenderChunk::new(blocks.drain(0..16).as_slice().to_vec(),working_dir.clone(),RenderChunkLocation::new(location.x,y as i32,location.z),block_registry.clone(),block_sides.clone()));
        }

        let aabb = AABB { min: vec3(location.x as f64 * WIDTH as f64 , 0.0, location.z as f64 * DEPTH as f64 ), max:  vec3(location.x as f64 * WIDTH as f64  + WIDTH as f64, 0.0, location.z as f64 * DEPTH as f64  + DEPTH as f64)};

        return Chunk {
            location,
            block_sides,
            aabb,
            render_chunks,
            size: 6,
            biome_data: biome_types
        }
    }

    fn generate_blocks(block_registry:Arc<Mutex<BlockRegistry>>,noise:Arc<OpenSimplex>,loc:&ChunkLocation, selector:Arc<BiomeSelector>,biome_types:&mut Vec<Vec<Arc<BiomeType>>>) -> Vec<Vec<Vec<Arc<BlockType>>>> {
        let mut blocks: Vec<Vec<Vec<Arc<BlockType>>>> = vec![vec![vec![block_registry.lock().unwrap().blocks["air"].clone();WIDTH];DEPTH];HEIGHT];

        for x in 0..WIDTH {
            for z in 0..DEPTH {

                let l = BlockLocation::new_from(x as i32,0,z as i32).get_global_values(loc);
                let normal_block_loc = BlockLocation::new_from(x as i32,0,z as i32);

                let biome = selector.select(&l);

                biome_types[x].push(biome.clone());

                biome.generator.generate_chunk_block(block_registry.clone(),noise.clone(),loc,&l,&normal_block_loc,&mut blocks);
            }
        }

        /*
        for x in 0..WIDTH {
            for z in 0..DEPTH {
                for y in 0..HEIGHT {
                    let biome_type = biome_types[x][z].clone();
                    let loc = BlockLocation::new_from(x as i32,y as i32,z as i32);
                    biome_type.generator.populate(blocks[y][x][z].clone(),&mut blocks,loc);
                }
            }
        }
         */

        return blocks;
    }

    fn sum_octave(num_iterations:i32,x:f64,y:f64,persistence:f64,scale:f64,low:f64,high:f64,noise_fn:&OpenSimplex) -> f64{
        let mut max_amp = 0.0;
        let mut amp = 1.0;
        let mut freq = scale;
        let mut noise = 0.0;

        for i in 0..num_iterations {
            noise += noise_fn.get([x * freq,y * freq]) * amp;
            max_amp += amp;
            amp *= persistence;
            freq *= 2.0;
        }

        noise /= max_amp;

        noise = noise * (high-low) / 2.0 + (high + low) / 2.0;

        return noise;
    }

    pub fn generate_meshes(&self, meshes:&mut HashMap<RenderChunkLocation,ChunkVertexBufferData>,group:ChunkGroup) {
        for c in self.render_chunks.iter() {
            meshes.insert(c.location.clone(),ChunkMesh::new_data(self, c, self.block_sides.clone(), &group));
        }
    }

    pub fn get_render_chunk_for_loc(&self, loc:&BlockLocation) -> Option<&RenderChunk> {
        let y = loc.y / crate::world::chunk::render_chunk::HEIGHT as i32;
        if y <= self.size - 1 && y > -1 {
            return Some(&self.render_chunks[y as usize]);
        } else {
            return None;
        }
    }

    pub fn get_render_chunk_for_loc_mut(&mut self, loc:&BlockLocation) -> Option<&mut RenderChunk> {
        let y = loc.y / crate::world::chunk::render_chunk::HEIGHT as i32;
        if y <=  self.size - 1 && y > -1{
            return Some(&mut self.render_chunks[y as usize]);
        } else {
            return None;
        }
    }

    pub fn get_render_chunk_mut(&mut self,loc:&RenderChunkLocation) -> Option<&mut RenderChunk> {
        if loc.y <= self.size - 1 {
            return Some(&mut self.render_chunks[loc.y as usize]);
        }

        return None;
    }

    pub fn is_surrounded_by_solid(&self, loc:&BlockLocation) -> bool {
        return self.is_air(&loc.up()) && self.is_air(&loc.down()) && self.is_air(&loc.north()) && self.is_air(&loc.east()) && self.is_air(&loc.south()) && self.is_air(&loc.west());
    }

    pub fn is_air(&self, loc: &BlockLocation) -> bool{
        let c = self.get_render_chunk_for_loc(loc);

        if c.is_some() {
            let c = c.unwrap();
            let loc = loc.get_render_chunk_values(c);
            return c.is_air(&loc);
        }

        return false;
    }

    pub fn is_transparent_and_self(&self, t:Arc<BlockType>,loc:&BlockLocation) -> bool{
        let c = self.get_render_chunk_for_loc(loc);

        if c.is_some() {
            let c = c.unwrap();
            let loc = loc.get_render_chunk_values(c);
            return c.is_transparent_and_self(t,&loc);
        }

        return true;
    }

    pub fn is_air_unsafe(&self, loc: &BlockLocation) -> bool{

        let c = self.get_render_chunk_for_loc(loc);

        if c.is_some() {
            let c = c.unwrap();
            let loc = loc.get_render_chunk_values(c);
            c.is_air_unsafe(&loc);
        }

        return false;
    }

    pub fn is_no_block_above(&self, loc: &BlockLocation) -> bool {
        for y in loc.y..HEIGHT as i32 {
            if self.is_air(&BlockLocation::new_from(loc.x, y+1, loc.z)) {
                return false;
            }
        }
        return true;
    }

    pub fn block_exists_in_chunk(&self,loc:&BlockLocation) -> bool {

        let c = self.get_render_chunk_for_loc(loc);

        if c.is_some() {
            let c = c.unwrap();
            let loc = loc.get_render_chunk_values(c);
            return c.block_exists_in_chunk(&loc);
        }

        return false;
    }

    pub fn get_block_at(&self, loc: &BlockLocation) -> Option<Arc<BlockType>>{

        let c = self.get_render_chunk_for_loc(loc);

        if c.is_some() {
            let c = c.unwrap();
            let loc = loc.get_render_chunk_values(c);
            return c.get_block_at(&loc);
        }

        return None;
    }

    pub unsafe fn set_block_at(&mut self, t: Arc<BlockType>, loc: &BlockLocation) {

        let c = self.get_render_chunk_for_loc_mut(loc);

        if c.is_some() {
            let c = c.unwrap();
            let loc = loc.get_render_chunk_values(c);

            c.set_block_at(t.clone(),&loc);
        }
    }

    pub fn save_real(&self,working_dir:String) {
        for c in self.render_chunks.iter() {
            c.save_real(working_dir.clone());
        }
    }
}

impl Save for Chunk {
    fn save(&self, file_path: String) {
        for c in self.render_chunks.iter() {
            c.save(format!("{}\\c{}T{}T{}.dat",file_path,c.location.x,c.location.y,c.location.z));
        }
    }
}