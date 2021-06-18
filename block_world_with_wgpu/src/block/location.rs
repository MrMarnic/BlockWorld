use nalgebra_glm::{TVec3, vec3, TMat4};
use crate::world::chunk::chunk::{Chunk, ChunkLocation};
use rust_game_library_wgpu::objects::aabb::AABB;
use crate::world::chunk::render_chunk::RenderChunk;
use std::sync::Arc;

#[derive(Clone,PartialEq)]
pub struct BlockLocation {
    pub x:i32,
    pub y:i32,
    pub z:i32
}

impl BlockLocation {
    pub fn null() -> BlockLocation {
        return BlockLocation {
            x: 0,
            y: 0,
            z: 0
        };
    }

    pub fn new_from(x:i32,y:i32,z:i32) -> BlockLocation {
        return BlockLocation {
            x,
            y,
            z
        };
    }

    pub fn new_from_round(x:f64,y:f64,z:f64) -> BlockLocation {
        return BlockLocation::new_from(BlockLocation::create_x(x),BlockLocation::create_x(y), z.ceil() as i32);
    }

    pub fn create_world_vec(&self) -> TVec3<f32>{
        return vec3(self.x as f32,self.y  as f32, self.z as f32 - 1.0);
    }

    pub fn create_x(x:f64) -> i32 {
        let mut new_x : i32 = 0;
        let amount: f64 = x / 0.5;
        if x > 0.0 {
            new_x = ((1.0+amount.abs())*0.5) as i32;
        }else {
            new_x = -((1.0+amount.abs())*0.5) as i32;
        }

        return new_x;
    }

    pub fn from_other_block_loc(loc:&BlockLocation) -> BlockLocation {
        return BlockLocation::new_from(loc.x,loc.y,loc.z);
    }

    pub fn subtract(&self, x:i32,y:i32,z:i32) -> BlockLocation {
        let new_loc = BlockLocation::new_from(self.x - x,self.y - y,self.z -z);
        return new_loc;
    }

    pub fn subtract_direction(&self, dir: &TVec3<f64>) -> BlockLocation {
        let new_loc = BlockLocation::new_from_round(self.x as f64 - dir.x,self.y as f64  - dir.y,self.z as f64  - dir.z);
        return new_loc;
    }

    pub fn add_direction(&self, dir: &TVec3<f64>) -> BlockLocation {
        let new_loc = BlockLocation::new_from_round(self.x as f64 + dir.x,self.y as f64  + dir.y,self.z as f64  + dir.z);
        return new_loc;
    }

    pub fn add(&self, x:i32,y:i32,z:i32) -> BlockLocation {
        let new_loc = BlockLocation::new_from(self.x + x,self.y + y,self.z + z);
        return new_loc;
    }

    pub fn get_chunk_values(&self, chunk_loc:&ChunkLocation) -> BlockLocation {
        return BlockLocation::new_from(self.x - chunk_loc.x * crate::world::chunk::chunk::WIDTH as i32, self.y, self.z - chunk_loc.z * crate::world::chunk::chunk::DEPTH as i32);
    }

    pub fn get_global_values(&self, chunk_loc:&ChunkLocation) -> BlockLocation {
        return BlockLocation::new_from(self.x + chunk_loc.x * crate::world::chunk::chunk::WIDTH as i32, self.y, self.z + chunk_loc.z * crate::world::chunk::chunk::DEPTH as i32);
    }

    pub fn get_render_chunk_values(&self,render_chunk:&RenderChunk) -> BlockLocation {
        return BlockLocation::new_from(self.x, self.y - render_chunk.location.y * crate::world::chunk::render_chunk::HEIGHT as i32, self.z);
    }

    pub fn up(&self) -> BlockLocation {
        let new_loc = BlockLocation::new_from(self.x,self.y + 1,self.z);

        return new_loc;
    }

    pub fn down(&self) -> BlockLocation {
        let new_loc = BlockLocation::new_from(self.x,self.y - 1,self.z);

        return new_loc;
    }

    pub fn north(&self) -> BlockLocation {
        let new_loc = BlockLocation::new_from(self.x,self.y,self.z - 1);

        return new_loc;
    }

    pub fn west(&self) -> BlockLocation {
        let new_loc = BlockLocation::new_from(self.x-1,self.y,self.z);

        return new_loc;
    }

    pub fn south(&self) -> BlockLocation {
        let new_loc = BlockLocation::new_from(self.x,self.y,self.z + 1);

        return new_loc;
    }

    pub fn east(&self) -> BlockLocation {
        let new_loc = BlockLocation::new_from(self.x+1,self.y,self.z);

        return new_loc;
    }

    pub fn get_neighbours(&self) -> Vec<BlockLocation> {
        return vec![self.up(),self.down(),self.north(),self.west(),self.south(),self.east()];
    }

    pub fn get_neighbours_and_self(&self) -> Vec<BlockLocation> {
        return vec![self.up(),self.down(),self.north(),self.west(),self.south(),self.east(),BlockLocation::from_other_block_loc(self)];
    }

    pub fn get_side_neighbours_and_self(&self) -> Vec<BlockLocation> {
        return vec![self.north(),self.west(),self.south(),self.east(),self.north().east(),self.south().east(),self.south().west(),self.north().west(),BlockLocation::from_other_block_loc(self)];
    }

    pub fn get_side_neighbours(&self) -> Vec<BlockLocation> {
        return vec![self.north(),self.west(),self.south(),self.east(),self.north().east(),self.south().east(),self.south().west(),self.north().west()];
    }

    pub fn get_surrounding_sides(&self) -> Vec<BlockLocation> {
        return vec![self.north(),self.west(),self.south(),self.east()];
    }

    pub fn get_surrounding_sides_and_self(&self) -> Vec<BlockLocation> {
        return vec![self.north(),self.west(),self.south(),self.east(),BlockLocation::from_other_block_loc(self)];
    }

    pub fn get_surrounding_sides_and_up(&self) -> Vec<BlockLocation> {
        return vec![self.north(),self.west(),self.south(),self.east(),self.up()];
    }

    pub fn get_abb(&self) -> AABB {
        let aabb: AABB = AABB::new(vec3(self.x as f64-0.5,self.y as f64-0.5,self.z as f64-1.0),vec3(self.x as f64+0.5,self.y as f64+0.5,self.z as f64));
        return aabb;
    }

    pub fn equals(&self,other:&BlockLocation) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z;
    }

    pub fn vec(&self) -> TVec3<i32> {
        return vec3(self.x,self.y,self.z);
    }

    pub fn vec_f32(&self) -> TVec3<f32> {
        return vec3(self.x as f32,self.y as f32,self.z as f32);
    }

    pub fn get_matrix(&self) -> TMat4<f32>{
        return nalgebra_glm::translation(&self.vec_f32())
    }

    pub fn is_outside_of(&self, chunk:&RenderChunk) -> bool {
        if self.x < chunk.location.x * crate::world::chunk::chunk::WIDTH as i32 + 1{
            return true;
        }
        if self.x > crate::world::chunk::chunk::WIDTH as i32 + chunk.location.x * crate::world::chunk::chunk::WIDTH as i32 - 1{
            return true;
        }
        if self.z < chunk.location.z * crate::world::chunk::chunk::DEPTH as i32{
            return true;
        }
        if self.z > crate::world::chunk::chunk::DEPTH as i32 + chunk.location.z * crate::world::chunk::chunk::DEPTH as i32 - 1{
            return true;
        }
        if self.y < chunk.location.y * crate::world::chunk::render_chunk::HEIGHT as i32{
            return true;
        }
        if self.y > crate::world::chunk::render_chunk::HEIGHT as i32 + chunk.location.y * crate::world::chunk::render_chunk::HEIGHT as i32 - 1{
            return true;
        }
        return false;
    }

    pub fn is_outside_of_x_z(&self, chunk:&RenderChunk) -> bool {
        if self.x < chunk.location.x * crate::world::chunk::chunk::WIDTH as i32{
            return true;
        }
        if self.x > crate::world::chunk::chunk::WIDTH as i32 + chunk.location.x * crate::world::chunk::chunk::WIDTH as i32 - 1{
            return true;
        }
        if self.z < chunk.location.z * crate::world::chunk::chunk::DEPTH as i32{
            return true;
        }
        if self.z > crate::world::chunk::chunk::DEPTH as i32 + chunk.location.z * crate::world::chunk::chunk::DEPTH as i32 - 1{
            return true;
        }
        return false;
    }

    pub fn is_outside_of_y(&self, chunk:&RenderChunk) -> bool {
        if self.y < chunk.location.y * crate::world::chunk::render_chunk::HEIGHT as i32{
            return true;
        }
        if self.y > crate::world::chunk::render_chunk::HEIGHT as i32 + chunk.location.y * crate::world::chunk::render_chunk::HEIGHT as i32 - 1{
            return true;
        }
        return false;
    }
}



#[derive(PartialEq,Debug,Clone)]
pub enum Direction {
    NORTH,SOUTH,EAST,WEST,UP,DOWN
}

