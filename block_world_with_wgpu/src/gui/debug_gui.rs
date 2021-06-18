use rust_game_library_wgpu::gui::gui::Gui;
use wgpu::{Device, RenderPass, Queue, DeviceType, Backend};
use rust_game_library_wgpu::engine::game_engine::GameEngine;
use rust_game_library_wgpu::scene::scene::Scene;
use rust_game_library_wgpu::text::gui_text::GuiText;
use rust_game_library_wgpu::objects::color::Color;
use nalgebra_glm::vec3;
use crate::scene::in_game_scene::InGameScene;
use winit::event::VirtualKeyCode;
use rust_game_library_wgpu::objects::camera::Camera;
use crate::world::world::World;
use noise::Seedable;

pub struct DebugGui {
    pub should_close:bool,
    text_backend: GuiText,
    text_adapter_info: GuiText,
    text_cpu_info: GuiText,
    text_vsync: GuiText,
    text_fps: GuiText,
    text_player_pos: GuiText,
    text_seed: GuiText
}

impl DebugGui {
    pub fn new(engine:&mut GameEngine,device:&Device,queue:&Queue,camera:&Camera,world:&World) -> DebugGui {
        let info = engine.adapter.get_info();
        let mut backend_name = String::new();

        match info.backend{
            Backend::Dx12 => {
                backend_name = "Dx12".to_string();
            },
            Backend::Dx11 => {
                backend_name = "Dx11".to_string();
            },
            Backend::Gl => {
                backend_name = "OpenGl".to_string();
            },
            Backend::Metal => {
                backend_name = "Metal".to_string();
            },
            Backend::Vulkan => {
                backend_name = "Vulkan".to_string();
            },
            Backend::BrowserWebGpu => {
                backend_name = "BrowserWebGpu".to_string();
            },
            _ => {
                backend_name = "Unknown".to_string();
            }
        }

        let mut gpu_type = String::new();

        match info.device_type{
            DeviceType::Cpu => {
                gpu_type = "CPU".to_string();
            },
            DeviceType::DiscreteGpu => {
                gpu_type = "DiscreteGpu".to_string();
            },
            DeviceType::IntegratedGpu => {
                gpu_type = "IntegratedGpu".to_string();
            },
            DeviceType::VirtualGpu => {
                gpu_type = "VirtualGpu".to_string();
            },
            DeviceType::Other => {
                gpu_type = "Other".to_string();
            }
        }

        let text_backend = GuiText::new(vec![format!("Backend: {}",backend_name)],engine.resource_loader.fonts["Minecraft"].clone(),Color::new(0,0,0),vec3(10.0,engine.game_window.height as f32 - 100.0,0.0),device,engine);
        let text_adapter_info = GuiText::new(vec![format!("Gpu: {} ({})",info.name.to_string(),gpu_type)],engine.resource_loader.fonts["Minecraft"].clone(),Color::new(0,0,0),vec3(10.0,engine.game_window.height as f32 - 130.0,0.0),device,engine);
        let text_cpu_info = GuiText::new(vec![format!("Cpu: {} ",engine.cpu_brand)],engine.resource_loader.fonts["Minecraft"].clone(),Color::new(0,0,0),vec3(10.0,engine.game_window.height as f32 - 160.0,0.0),device,engine);
        let text_vsync = GuiText::new(vec![format!("Vsync: {} ",engine.game_window.vsync)],engine.resource_loader.fonts["Minecraft"].clone(),Color::new(0,0,0),vec3(10.0,engine.game_window.height as f32 - 190.0,0.0),device,engine);
        let text_fps = GuiText::new(vec![format!("Fps: {} ",0)],engine.resource_loader.fonts["Minecraft"].clone(),Color::new(0,0,0),vec3(10.0,engine.game_window.height as f32 - 220.0,0.0),device,engine);
        let text_player_pos = GuiText::new(vec!["".to_string()],engine.resource_loader.fonts["Minecraft"].clone(),Color::new(0,0,0),vec3(10.0,engine.game_window.height as f32 - 250.0,0.0),device,engine);
        let text_seed = GuiText::new(vec![format!("Seed: {}",world.chunk_handler.seed)],engine.resource_loader.fonts["Minecraft"].clone(),Color::new(0,0,0),vec3(10.0,engine.game_window.height as f32 - 280.0,0.0),device,engine);


        text_backend.write(queue,camera,engine);
        text_adapter_info.write(queue,camera,engine);
        text_cpu_info.write(queue,camera,engine);
        text_vsync.write(queue,camera,engine);
        text_fps.write(queue,camera,engine);
        text_player_pos.write(queue,camera,engine);
        text_seed.write(queue,camera,engine);

        return DebugGui {
            should_close: false,
            text_backend,
            text_adapter_info,
            text_cpu_info,
            text_vsync,
            text_fps,
            text_player_pos,
            text_seed
        }
    }
}

impl Gui for DebugGui {
    fn process_input(&mut self, engine: &mut GameEngine, scene: &mut Box<dyn Scene>, device: &Device) {
        if engine.input_handler.is_key_clicked(VirtualKeyCode::F2) {
            self.should_close = true;
        }
    }

    fn update(&mut self, engine: &mut GameEngine, scene: &mut Box<dyn Scene>, queue: &Queue, device:&Device) {
        let scene = &scene.as_any().downcast_ref::<InGameScene>();

        if scene.is_some() {
            let scene = scene.unwrap();
            let camera = &scene.camera_2d;
            let player = &scene.player;

            self.text_fps.change_text(vec![format!("Fps: {}",engine.fps.to_string())],device);
            self.text_player_pos.change_text(vec![format!("x: {:.2} y: {:.2} z: {:.2}",player.camera.transform.pos.x,player.camera.transform.pos.y,player.camera.transform.pos.z)],device);

            /*
            engine.text_renderer.render_gui_text_instant(&mut self.text_backend,&mut engine.offset_handler.camera_offset,camera,queue);
            engine.text_renderer.render_gui_text_instant(&mut self.text_adapter_info,&mut engine.offset_handler.camera_offset,camera,queue);
            engine.text_renderer.render_gui_text_instant(&mut self.text_cpu_info,&mut engine.offset_handler.camera_offset,camera,queue);
            engine.text_renderer.render_gui_text_instant(&mut self.text_vsync,&mut engine.offset_handler.camera_offset,camera,queue);
            engine.text_renderer.render_gui_text_instant(&mut self.text_fps,&mut engine.offset_handler.camera_offset,camera,queue);
            engine.text_renderer.render_gui_text_instant(&mut self.text_player_pos,&mut engine.offset_handler.camera_offset,camera,queue);
             */
            //self.test.write(queue,camera,engine);
        }
    }

    fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>, engine: &'a GameEngine, scene: &'a dyn Scene, queue: &Queue) {

        let camera = &scene.as_any().downcast_ref::<InGameScene>();

        if camera.is_some() {
            let camera = &camera.unwrap().camera_2d;

            engine.text_renderer.begin(render_pass,camera);
            engine.text_renderer.render(&self.text_adapter_info,render_pass,camera);
            engine.text_renderer.render(&self.text_cpu_info,render_pass,camera);
            engine.text_renderer.render(&self.text_backend,render_pass,camera);
            engine.text_renderer.render(&self.text_vsync,render_pass,camera);
            engine.text_renderer.render(&self.text_fps,render_pass,camera);
            engine.text_renderer.render(&self.text_player_pos,render_pass,camera);
            engine.text_renderer.render(&self.text_seed,render_pass,camera);
        }
    }

    fn window_resized(&mut self, width: i32, height: i32, engine: &GameEngine, scene: &mut Box<dyn Scene>) {

    }

    fn close(&self, scene: &mut Box<dyn Scene>, engine: &mut GameEngine) {
        self.text_adapter_info.free_up(engine);
        self.text_cpu_info.free_up(engine);
        self.text_vsync.free_up(engine);
        self.text_backend.free_up(engine);
        self.text_fps.free_up(engine);
        self.text_player_pos.free_up(engine);
    }

    fn open(&mut self, scene: &mut Box<dyn Scene>) {

    }

    fn should_close(&self) -> bool {
        self.should_close
    }

    fn destroy(&mut self) {

    }

    fn release_mouse(&self) -> bool {
        false
    }

    fn get_id(&self) -> String {
        "debug_gui".to_string()
    }
}