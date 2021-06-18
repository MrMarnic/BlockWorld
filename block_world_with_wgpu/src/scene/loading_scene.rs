use rust_game_library_wgpu::scene::scene::Scene;
use wgpu::{Device, SwapChain, RenderPass, Queue, SwapChainDescriptor, CommandEncoder, SwapChainTexture};
use rust_game_library_wgpu::engine::game_engine::GameEngine;
use rust_game_library_wgpu::gui::gui_handler::GuiHandler;
use std::any::Any;
use std::sync::{Mutex, Arc};
use rust_game_library_wgpu::objects::texture_map::TextureMap;
use std::rc::Rc;
use winit::event::VirtualKeyCode;
use nalgebra_glm::vec3;
use rust_game_library_wgpu::objects::color::Color;
use rust_game_library_wgpu::text::gui_text::GuiText;
use crate::scene::main_menu_scene::MainMenuScene;
use crate::block::block_registry::BlockRegistry;
use rust_game_library_wgpu::objects::camera::Camera;
use crate::world::biome::biome_registry::BiomeRegistry;
use crate::world::biome::biome_selector::BiomeSelector;

pub struct LoadingScene {
    finished_music:usize,
    pub block_registry: Arc<Mutex<BlockRegistry>>,
    pub texture_map: Option<Rc<TextureMap>>,
    pub camera_2d:Option<Camera>,
    gui_text_loading: GuiText
}

impl LoadingScene {
    pub fn new(camera_2d:Camera,device:&Device,engine:&mut GameEngine) -> LoadingScene {
        let mut text = GuiText::new(vec!["Loading".to_string()],engine.resource_loader.fonts["Minecraft"].clone(),Color::new(0,0,0),vec3(10.0,17.0,0.0),device,engine);

        return LoadingScene { finished_music: 0, block_registry: Arc::new(Mutex::new(BlockRegistry::new())), texture_map: None, camera_2d: Some(camera_2d),gui_text_loading: text }
    }
}

impl Scene for LoadingScene {
    fn loaded(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {
        engine.audio_handler.load_all_audio_in_folder("assets\\audio".to_string(),engine.working_dir.clone());
        self.gui_text_loading.write(queue,self.camera_2d.as_ref().unwrap(),engine);
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
        self.camera_2d.as_mut().unwrap().load_up(queue);
        if engine.audio_handler.is_loading() {
            let mut progress = 0.0;
            let v = engine.audio_handler.get_sources_count() as f32/ engine.audio_handler.get_loading_max() as f32;
            progress = v;
            engine.color_renderer.render_color_queue(&mut engine.offset_handler.camera_offset,Color::new(214,32,32),&vec3(100.0 + progress * (engine.game_window.width as f32 - 110.0) * 0.5,20.0,0.0),&vec3(progress * (engine.game_window.width as f32 - 110.0) * 0.5,10.0,0.0),self.camera_2d.as_ref().unwrap(),queue);
            //engine.text_renderer.render_gui_text_instant(&mut self.gui_text_loading,&mut engine.offset_handler.camera_offset,self.camera_2d.as_ref().unwrap(),queue);
        } else {
            let tex_map = Rc::new(TextureMap::new(engine.resource_loader.get_texture("textures.png".to_string()),16));
            crate::block::block_registry::BlockRegistry::add_all_types(self.block_registry.clone(),tex_map.clone());

            self.texture_map = Some(tex_map);
            engine.scene_to_open = Some(Box::new(MainMenuScene::new(engine,device,self.camera_2d.take().unwrap(),self.block_registry.clone(),self.texture_map.take().unwrap())));
        }
    }

    fn render(&self, engine: &GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler, encoder: &mut CommandEncoder, frame: &SwapChainTexture) {
        if self.camera_2d.is_some() {
            let mut pass = engine.create_render_pass(encoder,frame);

            engine.color_renderer.finish(&mut pass,self.camera_2d.as_ref().unwrap(),0..engine.color_renderer.to_render.len());

            engine.text_renderer.begin(&mut pass,self.camera_2d.as_ref().unwrap());
            engine.text_renderer.render(&self.gui_text_loading,&mut pass,self.camera_2d.as_ref().unwrap());
        }
    }

    fn window_resized(&mut self, width: i32, height: i32, engine: &GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {
        self.camera_2d.as_mut().unwrap().update_aspect_with_size(width as f64,height as f64);
        self.camera_2d.as_mut().unwrap().load_up(queue);
    }

    fn handle_tick(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {

    }

    fn handle_second(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {

    }

    fn destroy(&mut self) {

    }

    fn close(&mut self, engine: &mut GameEngine) {
        self.gui_text_loading.free_up(engine);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}