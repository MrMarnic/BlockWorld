use rust_game_library_wgpu::scene::scene::Scene;
use wgpu::{Device, SwapChain, RenderPass, SwapChainDescriptor, Queue, CommandEncoder, SwapChainTexture};
use rust_game_library_wgpu::engine::game_engine::GameEngine;
use rust_game_library_wgpu::gui::gui_handler::GuiHandler;
use std::any::Any;
use rust_game_library_wgpu::pipeline::pipeline::RenderPipelineGroupBuilder;
use rust_game_library_wgpu::objects::camera::Camera;
use nalgebra_glm::vec3;
use crate::scene::main_menu_scene::MainMenuScene;
use crate::scene::loading_scene::LoadingScene;

pub struct GameInitScene {

}

impl Scene for GameInitScene {
    fn loaded(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {
        let texture_bind_group_layout = RenderPipelineGroupBuilder::empty().create_texture_bind_group_layout(device);

        engine.resource_loader.load_all_textures_in_folder("assets\\textures".to_string(),engine.working_dir.clone(),device,queue,&texture_bind_group_layout);
        engine.resource_loader.load_font("assets\\fonts\\Minecraft.fnt".to_string(),"Minecraft".to_string(),engine.resource_loader.get_texture("Minecraft.png".to_string()).clone(),engine.working_dir.clone(),device,queue,&texture_bind_group_layout);

        let camera2d = Camera::new_orto(engine.game_window.width,engine.game_window.height,vec3(0.0,0.0,0.0),device);
        let camera3d = Camera::new_perspective(90.0,engine.game_window.width as f64,engine.game_window.height as f64,0.1,1000.0,vec3(0.0,0.0,0.0),device);

        //let scene = MainMenuScene::new(engine,device,camera2d,camera3d);
        let scene = LoadingScene::new(camera2d,device,engine);

        engine.open_scene(Box::new(scene));
    }

    fn process_input(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {

    }

    fn update(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {

    }

    fn render(&self, engine: &GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler, encoder: &mut CommandEncoder, frame: &SwapChainTexture) {

    }

    fn window_resized(&mut self, width: i32, height: i32, engine: &GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {

    }

    fn handle_tick(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {

    }

    fn handle_second(&mut self, engine: &mut GameEngine, device: &Device, queue: &Queue, gui_handler: &mut GuiHandler) {

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