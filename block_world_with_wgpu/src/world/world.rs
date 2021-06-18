use std::sync::{Arc, Mutex};
use crate::block::block_type::BlockType;
use crate::block::block_sides::BlockSides;
use crate::world::chunk::chunk_handler::ChunkHandler;
use crate::block::location::BlockLocation;
use crate::player::player::Player;
use rust_game_library_wgpu::engine::game_engine::GameEngine;
use wgpu::{Device, Queue, RenderPass};
use crate::data::save::Save;
use crate::world::chunk::chunk_renderer::ChunkRenderer;
use rust_game_library_wgpu::objects::camera::Camera;
use std::rc::Rc;
use rust_game_library_wgpu::objects::texture_map::TextureMap;
use rust_game_library_wgpu::gui::gui_handler::GuiHandler;
use std::time::Instant;
use std::collections::HashMap;
use crate::world::chunk::chunk::ChunkLocation;
use crate::world::chunk::chunk_mesh::ChunkMesh;
use crate::world::biome::biome_registry::BiomeRegistry;
use crate::world::biome::biome_selector::BiomeSelector;

pub struct World {
    pub chunk_handler: ChunkHandler
}

impl World  {
    pub unsafe fn new(engine:&GameEngine,player:&Player, chunk_distance:i32, biome_reg: Arc<BiomeRegistry>, biome_selector: Arc<BiomeSelector>) -> World{

        std::fs::create_dir(format!("{}\\world",engine.working_dir));
        std::fs::create_dir(format!("{}\\world\\chunks",engine.working_dir));

        let mut chunk_handler = ChunkHandler::new(chunk_distance,player,biome_reg,biome_selector);
        let mut world = World {chunk_handler };

        return world;
    }

    pub fn is_air(&self,loc:&BlockLocation) -> bool {
        return self.chunk_handler.is_air(loc);
    }

    pub fn get_type(&self, loc:&BlockLocation) -> Option<Arc<BlockType>> {
        return self.chunk_handler.get_type(loc);
    }

    pub unsafe fn set_block_at(&mut self, t: Arc<BlockType>, loc:&BlockLocation, block_sides: Arc<BlockSides>,device:&Device) {
        self.chunk_handler.set_block_at(t,loc,block_sides,device);
    }

    pub fn finish_render<'a>(&'a self,chunk_renderer:&'a ChunkRenderer,render_pass:&mut RenderPass<'a>,camera:&'a Camera,texture_map:&'a Rc<TextureMap>) {
        chunk_renderer.begin_finish(render_pass,camera,&texture_map);
        for (loc,chunk) in &self.chunk_handler.draw_chunks {
            chunk_renderer.finish_instant(chunk,render_pass,camera);
        }
        /*
        for (loc,chunk) in &self.chunk_handler.loaded_chunks {
            chunk_renderer.finish_instant(chunk,render_pass,camera);
        }
         */
    }

    pub fn save(&self, file_path: String,world_saving_statistics:Arc<Mutex<WorldSavingStatistics>>) {
        std::fs::create_dir(file_path.clone());
        std::fs::create_dir(format!("{}\\chunks",file_path));
        world_saving_statistics.lock().unwrap().max_chunks = self.chunk_handler.loaded_chunks.len() as i32;
        for (loc,chunk) in self.chunk_handler.loaded_chunks.iter() {
            chunk.save(format!("{}\\chunks\\",file_path));
            world_saving_statistics.lock().unwrap().finished +=1;
        }
        world_saving_statistics.lock().unwrap().has_finished = true;
    }
}

pub struct WorldSavingStatistics {
    pub max_chunks: i32,
    pub finished: i32,
    pub has_finished: bool
}

impl WorldSavingStatistics {
    pub fn new() -> WorldSavingStatistics {
        return WorldSavingStatistics { max_chunks: 0, finished: 0, has_finished: false }
    }

    pub fn progress(&self) -> f32 {
        return self.finished as f32/self.max_chunks as f32;
    }
}