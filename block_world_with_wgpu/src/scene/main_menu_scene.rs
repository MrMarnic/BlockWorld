use rust_game_library_wgpu::scene::scene::Scene;
use wgpu::{Device, SwapChain, RenderPass, Queue, SwapChainDescriptor, Adapter, AdapterInfo, SwapChainTexture, CommandEncoder};
use rust_game_library_wgpu::engine::game_engine::GameEngine;
use rust_game_library_wgpu::gui::gui_handler::GuiHandler;
use std::any::Any;
use rust_game_library_wgpu::gui::gui_button::GuiButton;
use nalgebra_glm::vec3;
use rust_game_library_wgpu::gui::gui_component::GUIComponent;
use rust_game_library_wgpu::objects::camera::Camera;
use rust_game_library_wgpu::objects::texture_map::TextureMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use crate::block::block_registry::BlockRegistry;
use winit::event::VirtualKeyCode;
use crate::scene::world_loading_scene::WorldLoadingScene;
use crate::world::chunk::chunk_handler::ChunkLoadingStatistic;
use crate::world::biome::biome_registry::BiomeRegistry;
use crate::world::biome::biome_selector::BiomeSelector;

pub struct MainMenuScene {
    play_button: GuiButton,
    close_button: GuiButton,
    camera_2d:Option<Camera>,
    block_registry: Arc<Mutex<BlockRegistry>>,
    texture_map: Rc<TextureMap>
}

impl MainMenuScene {
    pub fn new(engine:&mut GameEngine,device:&Device,camera_2d:Camera,block_registry:Arc<Mutex<BlockRegistry>>,texture_map:Rc<TextureMap>) -> MainMenuScene{
        let g = GuiButton::new(engine.resource_loader.get_texture("button.png".to_string()),vec3((engine.game_window.width as f32 - 300.0 - 20.0) as f32,30.0,0.0),300,20,Box::new(|g|{

        }),"Play".to_string(),1,1,20,engine.resource_loader.fonts["Minecraft"].clone(),device,engine);

        let close = GuiButton::new(engine.resource_loader.get_texture("button.png".to_string()),vec3(320.0,30.0,0.0),300,20,Box::new(|g|{
            g.close();
        }),"Close".to_string(),1,1,20,engine.resource_loader.fonts["Minecraft"].clone(),device,engine);

        return MainMenuScene {
            play_button: g,
            close_button: close,
            camera_2d: Some(camera_2d),
            block_registry,
            texture_map
        }
    }
}

impl Scene for MainMenuScene {
    fn loaded(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {
        self.window_resized(engine.game_window.width,engine.game_window.height,engine,device,queue,gui_handler);
        self.play_button.gui_text.write(queue,self.camera_2d.as_ref().unwrap(),engine);
        self.close_button.gui_text.write(queue,self.camera_2d.as_ref().unwrap(),engine);
    }

    fn process_input(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {
        self.play_button.process_input(engine);
        self.close_button.process_input(engine);
        if engine.input_handler.is_key_clicked(VirtualKeyCode::F11) {
            if engine.game_window.is_full_screen() {
                engine.game_window.disable_full_screen();
            } else {
                engine.game_window.enable_windowed_full_screen();
            }
        }
        if self.play_button.is_clicked {
            engine.scene_to_open = Some(Box::new(WorldLoadingScene::new(engine,Arc::new(Mutex::new(ChunkLoadingStatistic::new())), self.block_registry.clone(),self.texture_map.clone(),self.camera_2d.take(),device)));
        }
    }

    fn update(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {
        engine.reset_render();
        if self.camera_2d.is_some() {
            self.camera_2d.as_ref().unwrap().load_up(queue);
            self.play_button.render(engine,queue,self.camera_2d.as_ref().unwrap());
            self.close_button.render(engine,queue,self.camera_2d.as_ref().unwrap());
        }
    }

    fn render(&self, engine: &GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler, encoder: &mut CommandEncoder, frame: &SwapChainTexture) {
        if self.camera_2d.is_some() {
            let mut pass = engine.create_render_pass(encoder,frame);

            engine.sprite_renderer.finish(&mut pass,self.camera_2d.as_ref().unwrap(),0..engine.sprite_renderer.to_render.len());
            engine.text_renderer.finish(&mut pass,self.camera_2d.as_ref().unwrap(),0..engine.text_renderer.to_render.len());

            engine.text_renderer.begin(&mut pass,self.camera_2d.as_ref().unwrap());
            engine.text_renderer.render(&self.play_button.gui_text,&mut pass,self.camera_2d.as_ref().unwrap());
            engine.text_renderer.render(&self.close_button.gui_text,&mut pass,self.camera_2d.as_ref().unwrap());
        }
    }


    fn window_resized(&mut self, width: i32, height: i32, engine: &GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {
        self.camera_2d.as_mut().unwrap().update_aspect_with_size(width as f64,height as f64);
        self.camera_2d.as_ref().unwrap().load_up(queue);

        let w = ((300.0/2560.0) * engine.game_window.width as f32) as i32;
        self.play_button.set_pos(vec3((engine.game_window.width as f32 - w as f32 - 20.0) as f32,30.0,0.0));
        self.play_button.set_size(w,20);
        self.close_button.set_size(w,20);
        self.close_button.set_pos(vec3(20.0 + w as f32,30.0,0.0));
    }

    fn handle_tick(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {

    }

    fn handle_second(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {

    }

    fn destroy(&mut self) {

    }

    fn close(&mut self, engine: &mut GameEngine) {
        self.close_button.gui_text.free_up(engine);
        self.play_button.gui_text.free_up(engine);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}