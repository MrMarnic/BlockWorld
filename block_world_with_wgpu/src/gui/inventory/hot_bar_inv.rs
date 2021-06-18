use crate::gui::inventory::inventory::Inventory;
use std::rc::Rc;
use std::sync::{Mutex, Arc};
use crate::block::block_registry::BlockRegistry;
use nalgebra_glm::vec3;
use crate::block::block_type::BlockType;
use rust_game_library_wgpu::objects::sprite::Sprite;
use rust_game_library_wgpu::engine::game_engine::GameEngine;
use rust_game_library_wgpu::objects::texture_object::TextureObject;
use rust_game_library_wgpu::objects::tex_coord::TexCoord;
use rust_game_library_wgpu::objects::texture_map::TextureMap;
use rust_game_library_wgpu::objects::camera::Camera;
use wgpu::Queue;
use winit::event::VirtualKeyCode;

pub struct HotBarInventory {
    pub inv: Inventory,
    pub slot_selector: Sprite,
    pub selected_slot:i32
}

impl HotBarInventory {
    pub fn new(engine:&GameEngine,slot_tex:Rc<TextureObject>,slot_select_tex:Rc<TextureObject>,block_registry:Arc<Mutex<BlockRegistry>>) -> HotBarInventory {
        let mut inv = Inventory::new(engine.game_window.width/2-10*32+10+16, 50, 10, 2, slot_tex, block_registry.clone());
        let slot_selector = Sprite::new(slot_select_tex,inv.slots.first().unwrap().sprite.transform.pos.clone(),vec3(33.0,33.0,0.0),false,TexCoord::default());

        let mut a = 0;
        for (id,b) in block_registry.clone().lock().unwrap().blocks.iter() {
            if !b.is_air {
                inv.set_block_in_slot(b.clone(),a as usize);
                a=a+1;
            }
        }

        return HotBarInventory { inv, slot_selector,selected_slot: 0 }
    }

    pub fn render(&self,engine:&mut GameEngine,tex_map:Rc<TextureMap>,camera:&Camera,queue:&Queue) {
        self.inv.render(engine,tex_map.clone(),camera,queue);
        engine.sprite_renderer.render_sprite_queue(&self.slot_selector,camera,queue,&mut engine.offset_handler.camera_offset);
    }

    pub fn window_resized(&mut self,engine:&GameEngine,width:i32,height:i32) {
        self.inv.set_pos(width/2-10*32+10+16, 50);
        self.select_slot(self.selected_slot);
    }

    pub fn process_input(&mut self,engine:&mut GameEngine) {
        if engine.input_handler.is_key_clicked(VirtualKeyCode::Key1) {
            self.select_slot(0);
        }
        if engine.input_handler.is_key_clicked(VirtualKeyCode::Key2) {
            self.select_slot(1);
        }
        if engine.input_handler.is_key_clicked(VirtualKeyCode::Key3) {
            self.select_slot(2);
        }
        if engine.input_handler.is_key_clicked(VirtualKeyCode::Key4) {
            self.select_slot(3);
        }
        if engine.input_handler.is_key_clicked(VirtualKeyCode::Key5) {
            self.select_slot(4);
        }
        if engine.input_handler.is_key_clicked(VirtualKeyCode::Key6) {
            self.select_slot(5);
        }
        if engine.input_handler.is_key_clicked(VirtualKeyCode::Key7) {
            self.select_slot(6);
        }
        if engine.input_handler.is_key_clicked(VirtualKeyCode::Key8) {
            self.select_slot(7);
        }
        if engine.input_handler.is_key_clicked(VirtualKeyCode::Key9) {
            self.select_slot(8);
        }
        if engine.input_handler.is_key_clicked(VirtualKeyCode::Key0) {
            self.select_slot(9);
        }
        if engine.input_handler.is_scroll_y() {
            if engine.input_handler.scroll_y > 0.0 {
                self.select_slot(self.selected_slot - 1);
            }else {
                self.select_slot(self.selected_slot + 1);
            }
        }
    }

    pub fn select_slot(&mut self,id:i32) {
        let mut real_id = id;
        if id > self.inv.width - 1 {
            real_id = id - self.inv.width;
        }else if id < 0{
            real_id = self.inv.width + id;
        }

        self.selected_slot = real_id;
        self.slot_selector.transform.set_translation(self.inv.slots[real_id as usize].pos.clone());
    }

    pub fn get_selected_block(&self) -> Arc<BlockType> {
        return self.inv.get_block_in_slot(self.selected_slot as usize);
    }
}