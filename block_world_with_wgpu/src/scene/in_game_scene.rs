use rust_game_library_wgpu::scene::scene::Scene;
use wgpu::{Device, SwapChain, RenderPass, Queue, SwapChainDescriptor, CommandEncoder, SwapChainTexture, BufferDescriptor};
use rust_game_library_wgpu::engine::game_engine::GameEngine;
use rust_game_library_wgpu::gui::gui_handler::GuiHandler;
use std::any::Any;
use std::sync::{Mutex, Arc};
use crate::block::block_registry::BlockRegistry;
use crate::world::world::World;
use crate::player::player::Player;
use rust_game_library_wgpu::objects::texture_map::TextureMap;
use std::rc::Rc;
use rust_game_library_wgpu::gui::gui_texture::GuiTexture;
use crate::block::block_sides::BlockSides;
use crate::world::chunk::chunk_renderer::ChunkRenderer;
use winit::event::VirtualKeyCode;
use nalgebra_glm::vec3;
use rust_game_library_wgpu::objects::tex_coord::TexCoord;
use crate::world::chunk::chunk::{ChunkLocation, Chunk};
use crate::world::chunk::chunk_mesh::ChunkMesh;
use rust_game_library_wgpu::objects::depth_texture::DepthTexture;
use rust_game_library_wgpu::objects::sprite::Sprite;
use rust_game_library_wgpu::objects::camera::Camera;
use std::time::Instant;
use crate::block::line_mesh_renderer::LineMeshRenderer;
use crate::block::line_mesh::LineMesh;
use rust_game_library_wgpu::objects::vertex_buffer::OnlyCoordsVertexBuffer;
use rust_game_library_wgpu::objects::vertex::OnlyCoordsVertex;
use wgpu::util::{DeviceExt, BufferInitDescriptor};
use crate::world::biome::biome_registry::BiomeRegistry;
use crate::world::biome::biome_selector::BiomeSelector;

pub struct InGameScene {
    pub block_registry: Arc<Mutex<BlockRegistry>>,
    pub world: World,
    pub player: Player,
    pub block_sides: Arc<BlockSides>,
    pub chunk_renderer: ChunkRenderer,
    pub line_mesh_renderer: LineMeshRenderer,
    pub texture_map: Rc<TextureMap>,
    pub crosshair: Sprite,
    pub selected_block_line_mesh : LineMesh,
    pub paused: bool,
    depth_texture:DepthTexture,
    pub camera_2d: Camera,
    pub biome_registry: Arc<BiomeRegistry>,
    pub biome_selector: Arc<BiomeSelector>
}

impl InGameScene {
    pub fn new(engine:&GameEngine,block_registry: Arc<Mutex<BlockRegistry>>,
               world: World,
               player: Player,
               block_sides: Arc<BlockSides>,
               texture_map:Rc<TextureMap>,
        device:&Device, sc: &(SwapChain, SwapChainDescriptor),camera_2d:Camera,biome_registry:Arc<BiomeRegistry>,biome_selector:Arc<BiomeSelector>) -> InGameScene {

                let line_mesh = LineMesh::new(OnlyCoordsVertexBuffer::new(device,vec![OnlyCoordsVertex::new(-0.505,0.505,-0.005),OnlyCoordsVertex::new(-0.505,-0.505,-0.005),
                                                                                      OnlyCoordsVertex::new(0.505,-0.505,-0.005),OnlyCoordsVertex::new(0.505,0.505,-0.005),
                                                                                      OnlyCoordsVertex::new(-0.505,0.505,1.005),OnlyCoordsVertex::new(-0.505,-0.505,1.005),
                                                                                      OnlyCoordsVertex::new(0.505,-0.505,1.005),OnlyCoordsVertex::new(0.505,0.505,1.005)],vec![0,1,1,2,2,3,3,0,
                                                                                                                                                                               0,4,3,7,1,5,2,6,
                                                                                                                                                                               4,5,6,7,5,6,7,4],false));

            return InGameScene {
                block_registry,
                world,
                player,
                block_sides,
                chunk_renderer: ChunkRenderer::new(engine.working_dir.clone(),device,&sc.1),
                line_mesh_renderer: LineMeshRenderer::new(engine.working_dir.clone(),device,&sc.1),
                texture_map,
                crosshair: Sprite::new(engine.resource_loader.get_texture("crosshair.png".to_string()),vec3(engine.game_window.width as f64/2.0,engine.game_window.height as f64/2.0,0.0),vec3(16.0,16.0,0.0),false,TexCoord::default()),
                selected_block_line_mesh: line_mesh,
                paused: false,
                depth_texture: DepthTexture::new(device,&sc.1),
                camera_2d,
                biome_registry,
                biome_selector
            };
        }
}

impl Scene for InGameScene {
    fn loaded(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {
        engine.game_window.take_cursor();
        engine.tick_handler_paused = false;
        //self.world.write().render(&self.chunk_renderer,engine,&self.player.camera,queue);
    }

    fn process_input(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {
        if engine.input_handler.is_key_clicked(VirtualKeyCode::F11) {
            if engine.game_window.is_full_screen() {
                engine.game_window.disable_full_screen();
            } else {
                engine.game_window.enable_windowed_full_screen();
            }
        }
        if !self.paused {
            self.player.process_input(engine,&mut self.world,device,gui_handler,queue,&self.camera_2d);
        }
    }

    fn update(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {
        if !self.paused {
            self.world.chunk_handler.update(&self.player,self.block_registry.clone(),self.block_sides.clone(),device,engine,queue);
            self.player.update(engine);
            let ray = self.player.create_block_ray();
            let block = ray.start(&self.world);
            self.player.selected_block = block;
        }

        engine.reset_render();
        self.player.camera.load_up(queue);
        self.camera_2d.load_up(queue);

        //self.world.write().render(&self.chunk_renderer,engine,&self.player.camera,queue);
        queue.write_buffer(&self.chunk_renderer.shader.light_pos_buffer,0,&rust_game_library_wgpu::objects::matrix_helper::get_bytes_from_vec3(&vec3(self.player.camera.transform.pos.x as f32, self.player.camera.transform.pos.y as f32 + 50.0, self.player.camera.transform.pos.z as f32)));
        self.player.render_selected_block(engine,&self.line_mesh_renderer,&self.world,&mut self.selected_block_line_mesh,queue);
        engine.sprite_renderer.render_sprite_queue(&self.crosshair,&self.camera_2d,queue,&mut engine.offset_handler.camera_offset);
        self.player.hot_bar_inv.render(engine,self.texture_map.clone(),&self.camera_2d,queue);
    }

    fn render(&self, engine: &GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler, encoder: &mut CommandEncoder, frame: &SwapChainTexture) {
        let w = &self.world;
        {
            let mut pass = engine.create_render_pass_with_depth(encoder,frame,&self.depth_texture);
            w.finish_render(&self.chunk_renderer,&mut pass,&self.player.camera,&self.texture_map);
            if self.selected_block_line_mesh.should_render {
                self.line_mesh_renderer.finish_instant(&self.selected_block_line_mesh,&mut pass,&self.player.camera);
            }
        }
        let mut pass = engine.create_render_pass_load(encoder,frame);
        engine.sprite_renderer.finish(&mut pass,&self.camera_2d,0..engine.sprite_renderer.to_render.len());
        gui_handler.render(&mut pass,engine,self,queue);
    }

    fn window_resized(&mut self, width: i32, height: i32, engine: &GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {
        self.player.camera.update_aspect_with_size(width as f64,height as f64);
        self.player.camera.load_up(queue);
        self.camera_2d.update_aspect_with_size(width as f64,height as f64);
        self.camera_2d.load_up(queue);
        self.depth_texture = DepthTexture::new(device,&engine.sc.1);
        self.crosshair.transform.set_translation(vec3(width as f64/2.0,height as f64/2.0,0.0));
        self.player.hot_bar_inv.window_resized(engine,width,height);
    }

    fn handle_tick(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {
        if !self.paused {
            self.player.handle_tick(engine,&mut self.world,device);
        }
    }

    fn handle_second(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {
        self.world.chunk_handler.update_on_second(&self.player,self.block_registry.clone(),self.block_sides.clone(),device,engine,queue);
    }

    fn destroy(&mut self) {

    }

    fn close(&mut self, engine: &mut GameEngine) {

    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}