use rust_game_library_wgpu::scene::scene::Scene;
use wgpu::{SwapChainTexture, Device, CommandEncoder, Queue};
use rust_game_library_wgpu::engine::game_engine::GameEngine;
use rust_game_library_wgpu::gui::gui_handler::GuiHandler;
use std::any::Any;
use parking_lot::RwLock;
use crate::world::world::{World, WorldSavingStatistics};
use std::sync::{Arc, Mutex};
use crate::block::block_registry::BlockRegistry;
use rust_game_library_wgpu::objects::texture_map::TextureMap;
use std::rc::Rc;
use winit::event::VirtualKeyCode;
use crate::scene::main_menu_scene::MainMenuScene;
use rust_game_library_wgpu::objects::camera::Camera;
use nalgebra_glm::vec3;
use rust_game_library_wgpu::objects::color::Color;
use rust_game_library_wgpu::text::gui_text::GuiText;

pub struct SavingScene {
    pub stats: Arc<Mutex<WorldSavingStatistics>>,
    pub block_registry:Arc<Mutex<BlockRegistry>>,
    pub tex_map:Rc<TextureMap>,
    pub end_game: bool,
    pub camera_2d:Option<Camera>,
    gui_text_saved_chunks: GuiText,
    gui_text_saving: GuiText,
    gui_text_percentage: GuiText
}

impl SavingScene {
    pub fn new(engine:&mut GameEngine,stats:Arc<Mutex<WorldSavingStatistics>>,block_registry:Arc<Mutex<BlockRegistry>>,tex_map:Rc<TextureMap>,end_game:bool,camera_2d: Camera,device:&Device) -> SavingScene{
        let gui_text_saved_chunks = GuiText::new(vec!["".to_string()],engine.resource_loader.fonts["Minecraft"].clone(),Color::new(0,0,0),vec3(15.0,engine.game_window.height as f32 - 30.0,0.0),device,engine);
        let gui_text_saving = GuiText::new(vec!["Saving".to_string()],engine.resource_loader.fonts["Minecraft"].clone(),Color::new(0,0,0),vec3(10.0,37.0,0.0),device,engine);
        let gui_text_percentage = GuiText::new(vec!["".to_string()],engine.resource_loader.fonts["Minecraft"].clone(),Color::new(0,0,0),vec3(15.0,10.0,0.0),device,engine);
        return SavingScene { stats, block_registry, tex_map, end_game, camera_2d: Some(camera_2d), gui_text_saved_chunks, gui_text_saving, gui_text_percentage }
    }
}

impl Scene for SavingScene {
    fn loaded(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {
        self.gui_text_percentage.write(queue,self.camera_2d.as_ref().unwrap(),engine);
        self.gui_text_saved_chunks.write(queue,self.camera_2d.as_ref().unwrap(),engine);
        self.gui_text_saving.write(queue,self.camera_2d.as_ref().unwrap(),engine);

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
        if self.stats.lock().unwrap().has_finished {
            if self.end_game {
                engine.close();
            } else {
                let m = MainMenuScene::new(engine,device,self.camera_2d.take().unwrap(),self.block_registry.clone(),self.tex_map.clone());
                engine.scene_to_open = Some(Box::new(m));
            }
        }
        engine.reset_render();
        if self.camera_2d.is_some() {
            self.camera_2d.as_ref().unwrap().load_up(queue);
            let progress = self.stats.lock().unwrap().progress();
            let f = self.stats.lock().unwrap().finished;
            let m = self.stats.lock().unwrap().max_chunks;
            self.gui_text_saved_chunks.change_text(vec![format!("Saved Chunks:{}/{}",f,m)],device);
            //engine.text_renderer.render_gui_text_instant(&mut self.gui_text_saved_chunks,&mut engine.offset_handler.camera_offset,self.camera_2d.as_ref().unwrap(),queue);
            //engine.text_renderer.render_gui_text_instant(&mut self.gui_text_saving,&mut engine.offset_handler.camera_offset,self.camera_2d.as_ref().unwrap(),queue);
            //engine.text_renderer.render_string(&format!("Saved Chunks:{}/{}",f,m),&vec3(15.0,engine.gui_camera.height as f32 - 30.0,0.0),&engine.gui_camera,engine.resource_loader.fonts["Minecraft"].clone(),Color::new(0,0,0));
            //engine.text_renderer.render_string(&"Saving".to_string(),&vec3(10.0,37.0,0.0),&engine.gui_camera,engine.resource_loader.fonts["Minecraft"].clone(),Color::new(0,0,0));
            engine.color_renderer.render_color_queue(&mut engine.offset_handler.camera_offset,Color::new(230,56,25),&vec3(55.0 + (progress * (engine.game_window.width as f32 - 75.0) * 0.5),18.0,0.0),&vec3(progress * (engine.game_window.width as f32 - 75.0) as f32 * 0.5,10.0,0.0),self.camera_2d.as_ref().unwrap(),queue);
            //engine.text_renderer.render_string(&format!("{}%",(progress * 100.0) as i32),&vec3(15.0,10.0,0.0),&engine.gui_camera,engine.resource_loader.fonts["Minecraft"].clone(),Color::new(0,0,0));
            self.gui_text_percentage.change_text(vec![format!("{}%",(progress * 100.0) as i32)],device);
            //engine.text_renderer.render_gui_text_instant(&mut self.gui_text_percentage,&mut engine.offset_handler.camera_offset,self.camera_2d.as_ref().unwrap(),queue);
        }
    }

    fn render(&self, engine: &GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler, encoder: &mut CommandEncoder, frame: &SwapChainTexture) {
        if self.camera_2d.is_some() {
            let mut pass = engine.create_render_pass(encoder,frame);
            engine.color_renderer.finish(&mut pass,self.camera_2d.as_ref().unwrap(),0..engine.color_renderer.to_render.len());

            engine.text_renderer.begin(&mut pass,self.camera_2d.as_ref().unwrap());
            engine.text_renderer.render(&self.gui_text_percentage,&mut pass,self.camera_2d.as_ref().unwrap());
            engine.text_renderer.render(&self.gui_text_saving,&mut pass,self.camera_2d.as_ref().unwrap());
            engine.text_renderer.render(&self.gui_text_saved_chunks,&mut pass,self.camera_2d.as_ref().unwrap());
        }
    }

    fn window_resized(&mut self, width: i32, height: i32, engine: &GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {
        if self.camera_2d.is_some() {
            self.camera_2d.as_mut().unwrap().update_aspect_with_size(width as f64,height as f64);
            self.camera_2d.as_ref().unwrap().load_up(queue);
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
        self.gui_text_saving.free_up(engine);
        self.gui_text_saved_chunks.free_up(engine);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}