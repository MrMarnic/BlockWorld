use rust_game_library_wgpu::gui::gui_button::GuiButton;
use rust_game_library_wgpu::engine::game_engine::GameEngine;
use nalgebra_glm::vec3;
use rust_game_library_wgpu::gui::gui::Gui;
use rust_game_library_wgpu::scene::scene::Scene;
use wgpu::{Queue, RenderPass, Device};
use rust_game_library_wgpu::gui::gui_component::GUIComponent;
use crate::scene::in_game_scene::InGameScene;
use crate::world::world::WorldSavingStatistics;
use std::sync::{Mutex, Arc};
use crate::data::save::Save;
use rust_game_library_wgpu::objects::color::Color;
use winit::event::VirtualKeyCode;
use crate::scene::saving_scene::SavingScene;
use rust_game_library_wgpu::objects::camera::Camera;

pub struct PauseGui {
    pub should_close: bool,
    continue_button: GuiButton,
    home_scree_button: GuiButton,
    close_button: GuiButton
}

impl PauseGui {
    pub fn new(engine:&mut GameEngine,device:&Device,queue:&Queue,camera:&Camera) -> PauseGui {
        let but_tex = engine.resource_loader.get_texture("button.png".to_string());
        let cb = GuiButton::new(but_tex.clone(),
                                vec3(((engine.game_window.width as f32 / 2.0) as i32) as f32,((engine.game_window.height as f32 / 2.0) as i32) as f32 + 100.0,0.0),200,20,Box::new(|g|{
                g.guis_to_close.push("pause_gui".to_string());
            }),"Continue Play".to_string(),1,1,20,engine.resource_loader.fonts["Minecraft"].clone(),device,engine);

        let hb = GuiButton::new(but_tex.clone(),
                                vec3(((engine.game_window.width as f32 / 2.0) as i32) as f32,((engine.game_window.height as f32 / 2.0) as i32) as f32,0.0),200,20,Box::new(|g|{

            }),"Home Screen".to_string(),1,1,20,engine.resource_loader.fonts["Minecraft"].clone(),device,engine);

        let clb = GuiButton::new(but_tex.clone(),
                                 vec3(((engine.game_window.width as f32 / 2.0) as i32) as f32,((engine.game_window.height as f32 / 2.0) as i32) as f32 - 100.0,0.0),200,20,Box::new(|g|{
            }),"Close Game".to_string(),1,1,20,engine.resource_loader.fonts["Minecraft"].clone(),device,engine);

        cb.gui_text.write(queue,camera,engine);
        hb.gui_text.write(queue,camera,engine);
        clb.gui_text.write(queue,camera,engine);

        return PauseGui {
            should_close: false,
            continue_button: cb,
            home_scree_button: hb,
            close_button: clb
        }
    }
}

impl Gui for PauseGui {
    fn process_input(&mut self, engine: &mut GameEngine, scene: &mut Box<dyn Scene>,device:&Device) {
        self.continue_button.process_input(engine);
        self.home_scree_button.process_input(engine);
        self.close_button.process_input(engine);
        if engine.input_handler.is_key_clicked(VirtualKeyCode::Escape) {
            self.should_close = true;
        }
        if self.home_scree_button.is_clicked {
            self.should_close = true;
            let scene =  scene.as_any_mut().downcast_mut::<InGameScene>().unwrap();

            let world = &mut scene.world;
            let breg = scene.block_registry.clone();
            let tex_map = scene.texture_map.clone();
            let player = &scene.player;
            let stats = Arc::new(Mutex::new(WorldSavingStatistics::new()));
            let path = format!("{}\\{}\\{}",engine.working_dir,"world","player.dat");
            let w_path = format!("{}\\{}",engine.working_dir,"world");

            player.save(path);
            let camera = scene.camera_2d.clone(device);
            engine.scene_to_open = Some(Box::new(SavingScene::new(engine,stats.clone(),breg.clone(),tex_map.clone(),false,camera,device)));
            engine.static_offset_handler.reset();
            *world.chunk_handler.should_loop.lock().unwrap() = false;
            /*
            std::thread::spawn(move ||{
                world.clone().read().save(w_path,stats.clone());
            });
             */
        }
        if self.close_button.is_clicked {

            let mut scene = scene.as_any_mut().downcast_mut::<InGameScene>().unwrap();

            let world = &mut scene.world;
            let breg = scene.block_registry.clone();
            let tex_map = scene.texture_map.clone();
            let player = &scene.player;
            let stats = Arc::new(Mutex::new(WorldSavingStatistics::new()));
            let path = format!("{}\\{}\\{}",engine.working_dir,"world","player.dat");
            let w_path = format!("{}\\{}",engine.working_dir,"world");

            player.save(path);
            let camera = scene.camera_2d.clone(device);
            engine.scene_to_open = Some(Box::new(SavingScene::new(engine,stats.clone(),breg.clone(),tex_map.clone(),true,camera,device)));
            *world.chunk_handler.should_loop.lock().unwrap() = false;
            /*
            std::thread::spawn(move ||{
                world.clone().read().save(w_path,stats.clone());
            });
             */
        }
    }

    fn update(&mut self, engine: &mut GameEngine, scene: &mut Box<dyn Scene>,queue:&Queue,device:&Device) {
        let camera = &scene.as_any().downcast_ref::<InGameScene>();

        if camera.is_some() {
            let camera  = &camera.unwrap().camera_2d;
            engine.color_renderer.render_color_queue(&mut engine.offset_handler.camera_offset,Color::new_with_a(36,36,36,0.5),&vec3(0.0,0.0,0.0),&vec3(engine.game_window.width as f32,engine.game_window.height as f32,0.0),camera,queue);

            self.continue_button.render(engine,queue,camera);
            self.home_scree_button.render(engine,queue,camera);
            self.close_button.render(engine,queue,camera);
        }
    }

    fn render<'a>(&'a self,render_pass:&mut RenderPass<'a>,engine:&'a GameEngine,scene:&'a dyn Scene,queue: &Queue) {
        let camera = &scene.as_any().downcast_ref::<InGameScene>();

        if camera.is_some() {
            let camera = &scene.as_any().downcast_ref::<InGameScene>().unwrap().camera_2d;
            engine.color_renderer.finish(render_pass,camera,0..1);
            engine.sprite_renderer.finish(render_pass,camera,1..engine.sprite_renderer.to_render.len());

            engine.text_renderer.begin(render_pass,camera);
            engine.text_renderer.render(&self.continue_button.gui_text,render_pass,camera);
            engine.text_renderer.render(&self.close_button.gui_text,render_pass,camera);
            engine.text_renderer.render(&self.home_scree_button.gui_text,render_pass,camera);
        }
    }

    fn window_resized(&mut self, width: i32, height: i32, engine: &GameEngine, scene: &mut Box<dyn Scene>) {
        self.continue_button.set_pos(vec3(((engine.game_window.width as f32 / 2.0) as i32) as f32,((engine.game_window.height as f32 / 2.0) as i32) as f32 + 100.0,0.0));
        self.home_scree_button.set_pos(vec3(((engine.game_window.width as f32 / 2.0) as i32) as f32,((engine.game_window.height as f32 / 2.0) as i32) as f32,0.0));
        self.close_button.set_pos(vec3(((engine.game_window.width as f32 / 2.0) as i32) as f32,((engine.game_window.height as f32 / 2.0) as i32) as f32 - 100.0,0.0));
    }

    fn close(&self, scene: &mut Box<dyn Scene>, engine: &mut GameEngine) {
        if scene.as_any_mut().downcast_mut::<InGameScene>().is_some() {
            scene.as_any_mut().downcast_mut::<InGameScene>().unwrap().paused = false;
            engine.game_window.take_cursor();
            engine.tick_handler_paused = false;
        }

        self.close_button.gui_text.free_up(engine);
        self.continue_button.gui_text.free_up(engine);
        self.home_scree_button.gui_text.free_up(engine);
    }

    fn open(&mut self, scene: &mut Box<dyn Scene>) {
        scene.as_any_mut().downcast_mut::<InGameScene>().unwrap().paused = true;
    }

    fn should_close(&self) -> bool {
        self.should_close
    }

    fn destroy(&mut self) {

    }

    fn release_mouse(&self) -> bool {
        true
    }

    fn get_id(&self) -> String {
        "pause_gui".to_string()
    }
}