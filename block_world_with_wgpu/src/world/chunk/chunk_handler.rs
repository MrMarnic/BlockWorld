use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::world::world::World;
use crate::block::block_registry::BlockRegistry;
use crate::block::block_sides::BlockSides;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use crate::block::block_type::BlockType;
use nalgebra_glm::vec3;
use crate::world::chunk::chunk::{Chunk, ChunkLocation};
use crate::world::chunk::chunk_mesh::ChunkMesh;
use crate::player::player::Player;
use crate::block::location::BlockLocation;
use rust_game_library_wgpu::objects::aabb::AABB;
use wgpu::{Device, Queue};
use rust_game_library_wgpu::objects::vertex_buffer::{VertexBuffer};
use rust_game_library_wgpu::objects::vertex_buffer_data::{VertexBufferData};
use rust_game_library_wgpu::engine::game_engine::GameEngine;
use rust_game_library_wgpu::objects::camera::Camera;
use crate::world::chunk::render_chunk::RenderChunkLocation;
use parking_lot::lock_api::RwLock;
use std::sync::mpsc::{channel, Sender,Receiver};
use threadpool::ThreadPool;
use crate::world::chunk::draw_chunk::DrawChunk;
use noise::{OpenSimplex, Seedable};
use rand::{thread_rng, Rng};
use crate::vertex::chunk_vertex_buffer_data::ChunkVertexBufferData;
use crate::vertex::chunk_vertex_buffer::ChunkVertexBuffer;
use crate::world::biome::biome_registry::BiomeRegistry;
use crate::world::biome::biome_selector::BiomeSelector;


pub struct ChunkHandler{
    pub loaded_chunks: HashMap<ChunkLocation, Arc<Chunk>>,
    pub draw_chunks: HashMap<RenderChunkLocation, DrawChunk>,
    chunk_distance: i32,
    chunk_loading_aabb: AABB,
    player_chunk_loc: ChunkLocation,
    pub should_loop :Arc<Mutex<bool>>,
    chunk_generator_pool: ThreadPool,
    meshing_pool: ThreadPool,
    tx_rx_chunk_to_pool: (Sender<ChunkCreationResult>,Receiver<ChunkCreationResult>),
    tx_rx_mesh_to_pool: (Sender<MeshCreationResult>,Receiver<MeshCreationResult>),
    simplex_noise: Arc<OpenSimplex>,
    pub seed: u32,
    pub generated_chunks_count: i32,
    pub has_generated_start: bool,
    pub to_generate: i32,
    pub biome_reg:Arc<BiomeRegistry>,
    pub biome_sec:Arc<BiomeSelector>
}

impl ChunkHandler {
    pub unsafe fn new(chunk_distance:i32, player:&Player,biome_reg:Arc<BiomeRegistry>,biome_sec:Arc<BiomeSelector>) -> ChunkHandler{
        let mut chunks = HashMap::new();

        let mut chunk_pool = ThreadPool::new(2);
        let mut mesh_pool = ThreadPool::new(8);

        let (tx,rx) = channel::<ChunkCreationResult>();

        let mut noise = OpenSimplex::new();
        let seed = thread_rng().gen_range(0..u32::MAX);

        println!("Seed: {}", seed);

        noise.set_seed(seed);

        return ChunkHandler { loaded_chunks: chunks,
            draw_chunks: HashMap::new(), chunk_distance,chunk_loading_aabb: ChunkHandler::create_chunk_loading_aabb(player, chunk_distance),player_chunk_loc: ChunkLocation::new(0, 0), should_loop: Arc::new(Mutex::new(true)),
            chunk_generator_pool: chunk_pool,
            meshing_pool: mesh_pool,
            tx_rx_chunk_to_pool: (tx, rx),
            tx_rx_mesh_to_pool: channel::<MeshCreationResult>(),
            simplex_noise: Arc::new(noise),
            seed,
            generated_chunks_count: 0,
            has_generated_start: false,
            to_generate: 0,
            biome_reg,
            biome_sec
        }
    }



    pub fn generate(&mut self,working_dir:String,chunk_loading_statistic:Arc<Mutex<ChunkLoadingStatistic>>,chunk_distance:i32,block_registry:Arc<Mutex<BlockRegistry>>, block_sides: Arc<BlockSides>,player_chunk_loc:ChunkLocation) {

        let min_x = -chunk_distance;
        let max_x = chunk_distance;
        let min_z = -chunk_distance;
        let max_z = chunk_distance;

        let max_size = (chunk_distance + chunk_distance + 1) * (chunk_distance + chunk_distance + 1);

        self.to_generate = max_size;

        let mut current = 0;

        chunk_loading_statistic.lock().unwrap().chunks_max = max_size;;

        for x in min_x..max_x+1 {
            for z in min_z..max_z+1 {

                let loc = ChunkLocation::new(x + player_chunk_loc.x,z + player_chunk_loc.z);

                self.create_chunk_queue(loc,working_dir.clone(),block_registry.clone(),block_sides.clone(),self.tx_rx_chunk_to_pool.0.clone());
            }
        }

    }

    pub fn generate_new_chunks(&self, working_dir:String,chunk_distance:i32,block_registry:Arc<Mutex<BlockRegistry>>, block_sides: Arc<BlockSides>) {
        let min_x = -chunk_distance;
        let max_x = chunk_distance;
        let min_z = -chunk_distance;
        let max_z = chunk_distance;

        let player_chunk_loc = self.player_chunk_loc.clone();

        for x in min_x..max_x+1 {
            for z in min_z..max_z+1 {

                let loc = ChunkLocation::new(x + player_chunk_loc.x,z + player_chunk_loc.z);

                if !self.chunk_is_loaded(&loc) {
                    self.create_chunk_queue(loc,working_dir.clone(),block_registry.clone(),block_sides.clone(),self.tx_rx_chunk_to_pool.0.clone());
                }
            }
        }
    }

    pub fn start(&mut self,working_dir:String,chunk_loading_stats:Arc<Mutex<ChunkLoadingStatistic>>,block_reg:Arc<Mutex<BlockRegistry>>,block_sides:Arc<BlockSides>,player_start_loc:ChunkLocation,freed_camera_offset: Arc<Mutex<Vec<u64>>>,chunk_distance:i32) {
        self.chunk_distance = chunk_distance;

        chunk_loading_stats.lock().unwrap().has_generated = true;

        self.generate(working_dir.clone(),chunk_loading_stats.clone(),chunk_distance,block_reg.clone(),block_sides.clone(),self.player_chunk_loc.clone());
        //self.has_generated_start = true;
    }

    pub fn update(&mut self, player: &Player, block_registry:Arc<Mutex<BlockRegistry>>, block_sides: Arc<BlockSides>,device:&Device,engine:&mut GameEngine,queue:&Queue) {

        self.chunk_loading_aabb = ChunkHandler::create_chunk_loading_aabb(player,self.chunk_distance);
        self.player_chunk_loc = self.get_chunk_loc_for_block(&player.get_block_pos());

        let result = self.tx_rx_chunk_to_pool.1.try_recv();
        if result.is_ok() {
            let result = result.unwrap();

            let mut chunk = result.chunk;

            let chunk_arc = Arc::new(chunk);

            self.loaded_chunks.insert(chunk_arc.location.clone(),chunk_arc.clone());

            let mt = MeshCreationTask { chunk: chunk_arc.clone(), group: ChunkGroup::new(chunk_arc.clone(),self)};

            let tx = self.tx_rx_mesh_to_pool.0.clone();

            self.meshing_pool.execute(move ||{
                ChunkHandler::remesh_chunk(mt,tx);
            });

            for loc in chunk_arc.location.get_surrounding_chunks().iter() {
                if let Some(ch) = self.loaded_chunks.get(&loc) {
                    self.remesh_chunk_queue(ch.clone());
                }
            }

            if !self.has_generated_start {
                self.generated_chunks_count += 1;
                if self.generated_chunks_count == self.to_generate {
                    self.has_generated_start = true;
                }
            }

            //self.remesh_chunk_queue(chunk_arc.clone());
        }

        let mesh_result = self.tx_rx_mesh_to_pool.1.try_recv();
        if mesh_result.is_ok() {
            let result = mesh_result.unwrap();

            for (loc,data) in result.data {
                if self.draw_chunks.contains_key(&loc) {
                    let mut dc = self.draw_chunks.get_mut(&loc).unwrap();
                    dc.mesh = Some(ChunkMesh { mesh: ChunkVertexBuffer::new_from_data(device,data) });
                } else {
                    if self.chunk_is_loaded(&loc.get_chunk_location()) {
                        let mut dc = DrawChunk::new(loc,engine);
                        dc.mesh = Some(ChunkMesh { mesh: ChunkVertexBuffer::new_from_data(device,data) });
                        dc.write(&player.camera,queue);
                        self.draw_chunks.insert(dc.loc.clone(),dc);
                    }
                }
            }
        }
    }

    pub fn update_on_second(&mut self, player: &Player, block_registry:Arc<Mutex<BlockRegistry>>, block_sides: Arc<BlockSides>,device:&Device,engine:&mut GameEngine,queue:&Queue) {
        if self.has_generated_start {
            let mut chunks_to_unload = vec![];

            let aabb = self.chunk_loading_aabb.clone();

            for (loc,chunk) in self.loaded_chunks.iter() {
                if !chunk.aabb.collision_test_no_y(&aabb) {
                    chunks_to_unload.push(chunk.location.clone());
                }
            }

            if !chunks_to_unload.is_empty() {
                for loc in chunks_to_unload.iter() {
                    let c = self.loaded_chunks.remove(loc).unwrap();

                    for rc in c.render_chunks.iter() {
                        if let Some(dc) = self.draw_chunks.remove((&rc.location)) {
                            engine.static_offset_handler.remove(dc.camera_offset as u64);
                        }
                    }
                }
            }

            for loc in chunks_to_unload.iter() {
                for c_loc in loc.get_surrounding_chunks().iter() {
                    if self.chunk_is_loaded(&c_loc) {
                        let c = self.loaded_chunks[c_loc].clone();
                        self.remesh_chunk_queue(c);
                    }
                }
            }

            chunks_to_unload.clear();

            self.generate_new_chunks(engine.working_dir.clone(),self.chunk_distance,block_registry.clone(),block_sides.clone());
        }
    }

    pub fn remesh_chunk_queue(&self, chunk:Arc<Chunk>) {

        let tx = self.tx_rx_mesh_to_pool.0.clone();

        let mt = MeshCreationTask { chunk: chunk.clone(), group: ChunkGroup::new(chunk,self) };

        self.meshing_pool.execute(move || {
            ChunkHandler::remesh_chunk(mt,tx);
        });
    }

    pub fn create_chunk_queue(&self,chunk_loc:ChunkLocation,working_dir:String,breg:Arc<Mutex<BlockRegistry>>,bsides:Arc<BlockSides>,tx:Sender<ChunkCreationResult>) {
        let n = self.simplex_noise.clone();
        let sec = self.biome_sec.clone();

        self.chunk_generator_pool.execute(move ||{
            let chunk = Chunk::new(working_dir,chunk_loc,breg.clone(),bsides.clone(),n,sec);

            let result = ChunkCreationResult { chunk };

            tx.send(result).unwrap();
        });
    }

    pub fn remesh_chunk(mt:MeshCreationTask,tx:Sender<MeshCreationResult>) {
        let chunk = mt.chunk;

        let mut meshes = HashMap::new();
        chunk.generate_meshes(&mut meshes,mt.group);

        let result = MeshCreationResult { data: meshes };

        tx.send(result).unwrap();
    }

    pub fn get_chunk_loc_for_block(&self, loc:&BlockLocation) -> ChunkLocation {
        let mut chunk_x = (loc.x as f32 / crate::world::chunk::chunk::WIDTH as f32).floor() as i32;
        let mut chunk_z = (loc.z as f32 / crate::world::chunk::chunk::DEPTH as f32).floor() as i32;

        return ChunkLocation {
            x: chunk_x,
            z: chunk_z
        };
    }

    pub fn get_chunk_for_block_xy(&self, x:i32, z:i32) -> Option<Arc<Chunk>> {
        let loc = ChunkLocation::new(x,z);

        if self.loaded_chunks.contains_key(&loc) {
            return Some(self.loaded_chunks.get(&loc).unwrap().clone());
        }

        return Option::None;
    }

    pub fn get_chunk_for_block(&self,loc:&BlockLocation) -> Option<Arc<Chunk>> {
        let mut chunk_x = (loc.x as f32 / crate::world::chunk::chunk::WIDTH as f32).floor() as i32;
        let mut chunk_z = (loc.z as f32 / crate::world::chunk::chunk::DEPTH as f32).floor() as i32;

        return self.get_chunk_for_block_xy(chunk_x,chunk_z);
    }

    pub fn chunk_is_loaded(&self,loc:&ChunkLocation) -> bool {
        return self.loaded_chunks.contains_key(loc);
    }

    pub fn get_type(&self, loc:&BlockLocation) -> Option<Arc<BlockType>> {

        let chunk = self.get_chunk_for_block(loc);

        if chunk.is_some() {
            let c = chunk.unwrap();

            let chunk_pos = loc.get_chunk_values(&c.location);

            return c.get_block_at(&chunk_pos);
        }

        return Option::None;
    }

    pub fn is_air(&self, loc:&BlockLocation) -> bool{
        let b = self.get_type(loc);

        if b.is_some() {
            return b.unwrap().is_air;
        }

        return true;
    }

    pub fn is_transparent(&self, loc:&BlockLocation) -> bool{
        let b = self.get_type(loc);

        if b.is_some() {
            return b.unwrap().is_transparent;
        }

        return true;
    }

    pub fn is_transparent_and_self(&self, t:Arc<BlockType>,loc:&BlockLocation) -> bool{
        let b = self.get_type(loc);

        if b.is_some() {
            let a = b.unwrap();
            return a.is_transparent && a.is_air && t.is_transparent || a.is_transparent && !t.is_transparent;
        }

        return true;
    }

    pub fn is_surrounded_by_solid(&self, loc:&BlockLocation) -> bool {
        for l in loc.get_neighbours().iter() {
            if self.is_air(l) {
                return false;
            }
        }

        return true;
    }

    pub unsafe fn set_block_at(&mut self, t: Arc<BlockType>, loc:&BlockLocation, block_sides: Arc<BlockSides>,device:&Device) {
        let mut chunk = self.get_chunk_for_block(loc);

        if chunk.is_some() {
            let mut arc = chunk.unwrap().clone();
            let arc_clone = arc.clone();
            let c = Arc::make_mut(&mut arc);

            let chunk_pos = loc.get_chunk_values(&arc_clone.location);

            c.set_block_at(t,&chunk_pos);



            //let start = std::time::SystemTime::now();

            //self.remesh_chunk_queue(self.loaded_chunks[&arc_clone.location].clone());

            let mut already_updated : Vec<RenderChunkLocation> = vec![];

            if let Some(rc) = c.get_render_chunk_for_loc(&chunk_pos) {
                let group = ChunkGroup::new(arc_clone.clone(),self);

                let mesh = ChunkMesh::new_data(c,rc,block_sides.clone(),&group);

                self.draw_chunks.get_mut(&rc.location).unwrap().mesh = Some(ChunkMesh { mesh: ChunkVertexBuffer::new_from_data(device,mesh) });

                already_updated.push(rc.location.clone());

            }

            *self.loaded_chunks.get_mut(&arc_clone.location).unwrap() = arc;

            for l in loc.get_neighbours_and_self() {
                let base_chunk = self.get_chunk_for_block(&l).unwrap();

                if let Some(rc) = base_chunk.get_render_chunk_for_loc(&l) {

                    if !already_updated.contains(&rc.location) {

                        already_updated.push(rc.location.clone());

                        let group = ChunkGroup::new(base_chunk.clone(),self);

                        let mesh = ChunkMesh::new_data(&*base_chunk,rc,block_sides.clone(),&group);

                        self.draw_chunks.get_mut(&rc.location).unwrap().mesh = Some(ChunkMesh { mesh: ChunkVertexBuffer::new_from_data(device,mesh) });
                    }
                }
            }

            /*
            for l in loc.get_neighbours_and_self() {
                if let Some(c) = self.get_chunk_for_block(&l) {
                    self.remesh_chunk_queue(c);
                }
            }
             */

            /*
            for l in loc.get_neighbours_and_self() {
                let mut con = true;


                if let Some(render_chunk) = self.get_chunk_for_block(&l).unwrap().get_render_chunk_for_loc(&l) {

                    for l in &already_updated {
                        if l.equals(&render_chunk.location) {
                            con = false;
                            break;
                        }
                    }

                    if !con {
                        continue;
                    }

                    already_updated.push(render_chunk.location.clone());

                    //let render_chunk = chu.get_render_chunk_for_loc(loc);

                    //let render_chunk = render_chunk.unwrap();
                    //self.meshes_to_generate.write().push(render_chunk.location.clone());

                    //let mesh = ChunkMesh::new(self, render_chunk, block_sides.clone(),true,device);

                    //let chn = self.get_chunk_for_block_mut(&l).unwrap().get_render_chunk_for_loc_mut(&l).unwrap();
                    //chn.chunk_mesh = Some(mesh);
                }
            }
             */

            //println!("{}ms",start.elapsed().unwrap().as_millis());

        }
    }

    fn create_chunk_loading_aabb(player : &Player, chunk_distance: i32) -> AABB {
        return AABB::new(vec3(player.camera.transform.pos.x - chunk_distance as f64 * crate::world::chunk::chunk::WIDTH as f64 - 8.0, 0.0, player.camera.transform.pos.z - chunk_distance as f64 * crate::world::chunk::chunk::DEPTH as f64 - 8.0),
                         vec3(player.camera.transform.pos.x + chunk_distance as f64 * crate::world::chunk::chunk::WIDTH as f64 + 8.0, 0.0, player.camera.transform.pos.z + chunk_distance as f64 * crate::world::chunk::chunk::DEPTH as f64 + 8.0));
    }
}

pub struct ChunkLoadingStatistic {
    pub chunks_max:i32,
    pub chunks_generated:i32,
    pub meshes_generated:i32,
    pub has_generated:bool
}

impl ChunkLoadingStatistic {
    pub fn new() -> ChunkLoadingStatistic {
        return ChunkLoadingStatistic {
            chunks_max: 0,
            chunks_generated: 0,
            meshes_generated: 0,
            has_generated: false
        }
    }
}

pub struct ChunkCreationTask {
    pub loc: ChunkLocation
}

pub struct MeshCreationTask {
    pub chunk: Arc<Chunk>,
    pub group: ChunkGroup
}

pub struct ChunkGroup {
    pub chunks: HashMap<ChunkLocation,Arc<Chunk>>,
    pub base: ChunkLocation
}

impl ChunkGroup {
    pub fn new(base:Arc<Chunk>,chunk_handler:&ChunkHandler) -> Self {

        let mut chunks = HashMap::new();

        let n = chunk_handler.loaded_chunks.get(&base.location.north());
        if n.is_some() {
            let n = n.unwrap().clone();
            chunks.insert(n.location.clone(),n);
        }

        let e = chunk_handler.loaded_chunks.get(&base.location.east());
        if e.is_some() {
            let e = e.unwrap().clone();
            chunks.insert(e.location.clone(),e);
        }

        let s = chunk_handler.loaded_chunks.get(&base.location.south());
        if s.is_some() {
            let s = s.unwrap().clone();
            chunks.insert(s.location.clone(),s);
        }

        let w = chunk_handler.loaded_chunks.get(&base.location.west());
        if w.is_some() {
            let w = w.unwrap().clone();
            chunks.insert(w.location.clone(),w);
        }

        return Self {
            chunks,
            base: base.location.clone()
        }
    }

    pub fn get_chunk_for_block(&self,loc:&BlockLocation) -> Option<Arc<Chunk>> {
        let mut chunk_x = (loc.x as f32 / crate::world::chunk::chunk::WIDTH as f32).floor() as i32;
        let mut chunk_z = (loc.z as f32 / crate::world::chunk::chunk::DEPTH as f32).floor() as i32;

        let chunk_loc = ChunkLocation::new(chunk_x,chunk_z);

        if let Some(chunk) = self.chunks.get(&chunk_loc) {
            return Some(chunk.clone())
        }

        return None
    }

    pub fn is_transparent_and_self(&self, t:Arc<BlockType>,loc:&BlockLocation) -> bool{

        if let Some(chunk) = self.get_chunk_for_block(loc) {
            let n_loc = loc.get_chunk_values(&chunk.location);

            let b = chunk.get_block_at(&n_loc);

            if b.is_some() {
                let a = b.unwrap();
                return a.is_transparent && a.is_air && t.is_transparent || a.is_transparent && !t.is_transparent;
            }
        }

        return true;
    }
}

pub struct ChunkCreationResult {
    pub chunk: Chunk
}

pub struct MeshCreationResult {
    pub data: HashMap<RenderChunkLocation,ChunkVertexBufferData>
}