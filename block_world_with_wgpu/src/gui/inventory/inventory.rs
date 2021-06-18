use crate::gui::inventory::slot::Slot;
use std::rc::Rc;
use std::sync::{Mutex, Arc};
use crate::block::block_registry::BlockRegistry;
use nalgebra_glm::vec3;
use crate::block::block_type::BlockType;
use wgpu::{Texture, Queue};
use rust_game_library_wgpu::objects::sprite::Sprite;
use rust_game_library_wgpu::objects::texture_object::TextureObject;
use rust_game_library_wgpu::objects::tex_coord::TexCoord;
use rust_game_library_wgpu::engine::game_engine::GameEngine;
use rust_game_library_wgpu::objects::texture_map::TextureMap;
use rust_game_library_wgpu::objects::camera::Camera;

pub struct Inventory {
    pub slots: Vec<Slot>,
    pub width: i32,
    pub height: i32
}

impl Inventory {
    pub fn render(&self,engine:&mut GameEngine,texture_map:Rc<TextureMap>,camera:&Camera,queue:&Queue) {
        for slot in self.slots.iter() {
            slot.render(engine,camera,queue);
        }

        for slot in self.slots.iter() {
            slot.render_content(engine,camera,queue,texture_map.texture.clone());
        }
        /*
        texture_map.texture.bind();
        for slot in self.slots.iter() {
            slot.render_content(engine);
        }
         */
    }

    pub fn new(x:i32, y:i32, width:i32, height:i32, slot_texture:Rc<TextureObject>, block_registry:Arc<Mutex<BlockRegistry>>) -> Inventory {
        let mut slots = vec![];

        for w in 0..width {
            for h in 0..height {
                slots.push(Slot::new(Sprite::new(slot_texture.clone(),vec3((x+w*32*2+w*2) as f64,(y+h*32*2+h*2) as f64,0.0),vec3(32.0,32.0,0.0),false,TexCoord::default()),block_registry.lock().unwrap().blocks["air"].clone()));
            }
        }

        return Inventory {slots,width,height};
    }

    pub fn set_pos(&mut self,x:i32,y:i32) {
        for w in 0..self.width {
            for h in 0..self.height {
                self.slots[(w + h * self.width) as usize].sprite.transform.set_translation(vec3((x+w*32*2+w*2) as f64,(y+h*32*2+h*2) as f64,0.0));
                self.slots[(w + h * self.width) as usize].pos = self.slots[(w + h * self.width) as usize].sprite.transform.pos.clone();
            }
        }
    }

    pub fn set_block_in_slot(&mut self,block:Arc<BlockType>,id:usize) {
        self.slots[id].set_block(block);
    }

    pub fn get_block_in_slot(&self,slot:usize) -> Arc<BlockType> {
        return self.slots[slot].block.clone();
    }
}