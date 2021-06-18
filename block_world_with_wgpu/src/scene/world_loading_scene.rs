use std::any::Any;
use std::sync::{Arc, Mutex};
use crate::world::chunk::chunk_handler::ChunkLoadingStatistic;
use nalgebra_glm::{vec3, vec4};
use crate::block::block_registry::BlockRegistry;
use crate::player::player::Player;
use crate::world::world::World;
use crate::block::block_sides::BlockSides;
//use crate::scene::in_game_scene::InGameScene;
use std::rc::Rc;
use rust_game_library_wgpu::objects::texture_map::TextureMap;
use rust_game_library_wgpu::scene::scene::Scene;
use wgpu::{Device, SwapChain, RenderPass, SwapChainDescriptor, Queue, SwapChainTexture, CommandEncoder};
use rust_game_library_wgpu::engine::game_engine::GameEngine;
use rust_game_library_wgpu::gui::gui_handler::GuiHandler;
use rust_game_library_wgpu::objects::camera::Camera;
use winit::event::VirtualKeyCode;
use rust_game_library_wgpu::objects::color::Color;
use rust_game_library_wgpu::text::gui_text::GuiText;
use crate::scene::in_game_scene::InGameScene;
use crate::world::biome::biome_registry::BiomeRegistry;
use crate::world::biome::biome_selector::BiomeSelector;

pub struct WorldLoadingScene {
    pub chunk_loading_statistic: Arc<Mutex<ChunkLoadingStatistic>>,
    pub block_registry: Arc<Mutex<BlockRegistry>>,
    pub world: Option<World>,
    pub player: Option<Player>,
    pub block_sides: Arc<BlockSides>,
    pub texture_map: Rc<TextureMap>,
    pub camera_2d: Option<Camera>,
    gui_text_generating_chunks: GuiText,
    gui_text_generating_meshes: GuiText,
    gui_text_percentage: GuiText,
    pub biome_registry: Arc<BiomeRegistry>,
    pub biome_selector: Arc<BiomeSelector>
}

impl WorldLoadingScene {
    fn calculate_progress(&self) -> f32 {

        let c_generated = self.chunk_loading_statistic.lock().unwrap().chunks_generated as f32;
        let c_max = self.chunk_loading_statistic.lock().unwrap().chunks_max as f32;
        let m_generated = self.chunk_loading_statistic.lock().unwrap().meshes_generated as f32;

        return (c_generated / c_max +
            m_generated / c_max) * 0.5;
    }

    pub fn new(engine:&mut GameEngine,chunk_loading_statistic: Arc<Mutex<ChunkLoadingStatistic>>,b_reg:Arc<Mutex<BlockRegistry>>,texture_map:Rc<TextureMap>,camera_2d: Option<Camera>,device:&Device) -> WorldLoadingScene{
        unsafe {

            let b_sides = Arc::new(BlockSides::new());
            let mut p = Player::new(Camera::new_perspective(90.0,camera_2d.as_ref().unwrap().width,camera_2d.as_ref().unwrap().height,0.01,1000.0,vec3(0.0,25.0,0.0),device),engine,b_reg.clone(),b_sides.clone());
            p.camera.transform.acitvate_interpolation();

            let gui_text_generating_chunks = GuiText::new(vec!["".to_string()],engine.resource_loader.fonts["Minecraft"].clone(),Color::new(0,0,0),vec3(15.0,camera_2d.as_ref().unwrap().height as f32 - 30.0,0.0),device,engine);
            let gui_text_generating_meshes = GuiText::new(vec!["".to_string()],engine.resource_loader.fonts["Minecraft"].clone(),Color::new(0,0,0),vec3(15.0,camera_2d.as_ref().unwrap().height as f32 - 60.0,0.0),device,engine);
            let gui_text_percentage = GuiText::new(vec!["".to_string()],engine.resource_loader.fonts["Minecraft"].clone(),Color::new(0,0,0),vec3(15.0,10.0,0.0),device,engine);

            let mut biome_registry = BiomeRegistry::new();
            biome_registry.add_all_types();

            let breg_arc = Arc::new(biome_registry);

            let selector = Arc::new(BiomeSelector::new(breg_arc.clone()));

            return WorldLoadingScene {chunk_loading_statistic,
                block_registry: b_reg,
                world: Some(World::new(engine,&p,1,breg_arc.clone(),selector.clone())),
                player: Some(p),
                block_sides: b_sides,
                texture_map: texture_map.clone(),
                camera_2d,
                gui_text_generating_chunks,
                gui_text_generating_meshes,
                gui_text_percentage,
                biome_registry: breg_arc,
                biome_selector: selector
            }
        }
    }
}

impl Scene for WorldLoadingScene {
    fn loaded(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {
        let world = self.world.as_mut().unwrap();
        world.chunk_handler.start(engine.working_dir.clone(),self.chunk_loading_statistic.clone(),self.block_registry.clone(),self.block_sides.clone(),world.chunk_handler.get_chunk_loc_for_block(&self.player.as_ref().unwrap().get_block_pos()),engine.static_offset_handler.to_remove.clone(),4);
        self.gui_text_generating_chunks.write(queue,self.camera_2d.as_ref().unwrap(),engine);
        self.gui_text_generating_meshes.write(queue,self.camera_2d.as_ref().unwrap(),engine);
        self.gui_text_percentage.write(queue,self.camera_2d.as_ref().unwrap(),engine);

    }

    fn process_input(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {
        if engine.input_handler.is_key_clicked(VirtualKeyCode::F11) {
            if engine.game_window.is_full_screen() {
                engine.game_window.disable_full_screen();
            } else {
                engine.game_window.enable_windowed_full_screen();
            }
        }
    }

    fn update(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {
        engine.reset_render();
        if self.camera_2d.is_some() {
            self.camera_2d.as_mut().unwrap().load_up(queue);
            if self.chunk_loading_statistic.lock().unwrap().has_generated {
                unsafe {
                    self.world.as_mut().unwrap().chunk_handler.update(&self.player.as_ref().unwrap(),self.block_registry.clone(),self.block_sides.clone(),device,engine,queue);
                    engine.scene_to_open = Some(Box::new(InGameScene::new(engine,self.block_registry.clone(),self.world.take().unwrap(),self.player.take().unwrap(),self.block_sides.clone(),self.texture_map.clone(),device,&engine.sc,self.camera_2d.take().unwrap(),self.biome_registry.clone(),self.biome_selector.clone())));
                    return;
                }
            }
            let c_generated = self.chunk_loading_statistic.lock().unwrap().chunks_generated as f32;
            let c_max = self.chunk_loading_statistic.lock().unwrap().chunks_max as f32;
            let m_generated = self.chunk_loading_statistic.lock().unwrap().meshes_generated as f32;
            engine.color_renderer.render_color_queue(&mut engine.offset_handler.camera_offset,Color::new(256,256,256),&vec3(0.0,0.0,0.0),&vec3(self.camera_2d.as_ref().unwrap().width as f32,self.camera_2d.as_ref().unwrap().height as f32,0.0),self.camera_2d.as_ref().unwrap(),queue);
            self.gui_text_generating_chunks.change_text(vec![format!("Generating Chunks:{}/{}",c_generated,c_max)],device);
            self.gui_text_generating_meshes.change_text(vec![format!("Generating Chunk Meshes:{}/{}",m_generated,c_max)],device);
            //engine.text_renderer.render_gui_text_instant(&mut self.gui_text_generating_chunks,&mut engine.offset_handler.camera_offset,self.camera_2d.as_ref().unwrap(),queue);
            //engine.text_renderer.render_gui_text_instant(&mut self.gui_text_generating_meshes,&mut engine.offset_handler.camera_offset,self.camera_2d.as_ref().unwrap(),queue);
            let progress = self.calculate_progress();
            self.gui_text_percentage.change_text(vec![format!("{}%",(progress * 100.0) as i32)],device);
            //engine.text_renderer.render_gui_text_instant(&mut self.gui_text_percentage,&mut engine.offset_handler.camera_offset,self.camera_2d.as_ref().unwrap(),queue);
            engine.color_renderer.render_color_queue(&mut engine.offset_handler.camera_offset,Color::new(230,56,25),&vec3(55.0 + (progress * (engine.game_window.width as f32 - 75.0) * 0.5),18.0,0.0),&vec3(progress * (engine.game_window.width  as f32 - 75.0) as f32 * 0.5,10.0,0.0),self.camera_2d.as_ref().unwrap(),queue);

        }
    }

    fn render(&self, engine: &GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler, encoder: &mut CommandEncoder, frame: &SwapChainTexture) {
        if self.camera_2d.is_some() {

            let mut pass = engine.create_render_pass(encoder,frame);
            engine.color_renderer.finish(&mut pass,self.camera_2d.as_ref().unwrap(),0..engine.color_renderer.to_render.len());

            engine.text_renderer.begin(&mut pass,self.camera_2d.as_ref().unwrap());
            engine.text_renderer.render(&self.gui_text_generating_chunks,&mut pass,self.camera_2d.as_ref().unwrap());
            engine.text_renderer.render(&self.gui_text_generating_meshes,&mut pass,self.camera_2d.as_ref().unwrap());
            engine.text_renderer.render(&self.gui_text_percentage,&mut pass,self.camera_2d.as_ref().unwrap());
        }
    }

    fn window_resized(&mut self, width: i32, height: i32, engine: &GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {
        if self.camera_2d.is_some() {
            self.camera_2d.as_mut().unwrap().update_aspect_with_size(width as f64,height as f64);
            self.camera_2d.as_mut().unwrap().load_up(queue);
        }
    }

    fn handle_tick(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {

    }

    fn handle_second(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {

    }

    fn destroy(&mut self) {

    }

    fn close(&mut self, engine: &mut GameEngine) {
        self.gui_text_percentage.free_up(engine);
        self.gui_text_generating_meshes.free_up(engine);
        self.gui_text_generating_chunks.free_up(engine);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}