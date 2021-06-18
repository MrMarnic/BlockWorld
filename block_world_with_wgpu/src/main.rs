#![windows_subsystem = "windows"]

use rust_game_library_wgpu::engine::game_window::GameWindow;
use rust_game_library_wgpu::objects::color::Color;
use rust_game_library_wgpu::scene::scene_handler::SceneHandler;
use futures::executor::block_on;
use rust_game_library_wgpu::engine::game_events::GameEvents;
use crate::scene::game_init_scene::GameInitScene;
use wgpu::BackendBit;

pub mod scene;
pub mod block;
pub mod world;
pub mod player;
pub mod data;
pub mod gui;
pub mod vertex;

fn main() {
    let game_window = GameWindow::new("BlockWorld".to_string(),1920,1080,200,200,Color::new(255,255,255),false ,2);
    let game_events = GameEvents::new();
    let mut scene_handler = SceneHandler::new();
    scene_handler.opened_scene = Box::new(GameInitScene {});
    block_on(game_events.run(scene_handler,game_window,50,BackendBit::VULKAN));
}
