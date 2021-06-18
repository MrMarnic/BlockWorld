use crate::block::block_type::BlockType;
use std::sync::Arc;
use nalgebra_glm::{TVec3, vec3};
use rust_game_library_wgpu::objects::sprite::Sprite;
use rust_game_library_wgpu::engine::game_engine::GameEngine;
use wgpu::{RenderPass, Device, Queue};
use rust_game_library_wgpu::objects::camera::Camera;
use std::rc::Rc;
use rust_game_library_wgpu::objects::texture_object::TextureObject;
use rust_game_library_wgpu::objects::tex_coord::TexCoord;

pub struct Slot {
    pub sprite: Sprite,
    pub block: Arc<BlockType>,
    pub pos: TVec3<f64>,
    size: TVec3<f32>
}

impl Slot {
    pub fn render(&self,engine:&mut GameEngine,camera:&Camera,queue:&Queue) {
        engine.sprite_renderer.render_sprite_queue(&self.sprite,camera,queue,&mut engine.offset_handler.camera_offset);
    }

    pub fn finish<'a>(&'a self,render_pass:&mut RenderPass<'a>,engine:&'a GameEngine,device:&Device,queue:&Queue) {
        //engine.sprite_renderer.fi
    }

    pub fn render_content(&self,engine:&mut GameEngine,camera:&Camera,queue:&Queue,texture:Rc<TextureObject>) {
        //engine.sprite_renderer.render_sprite_queue(&self.sprite,camera,queue,&mut engine.offset_handler.camera_offset);
        if !self.block.is_air {
            engine.sprite_renderer.render_texture_with_tex_coords_queue(texture.clone(), &vec3(self.pos.x as f32, self.pos.y as f32, self.pos.z as f32), &self.size, camera, &self.block.tex_coords.south, queue, &mut engine.offset_handler.camera_offset);
        }
    }

    pub fn finish_content<'a>(&'a self,render_pass:&mut RenderPass<'a>,engine:&'a mut GameEngine,device:&Device,queue:&Queue,camera:&Camera) {
        if !self.block.is_air {
            //engine.sprite_renderer.render_with_tex_coords()
        }
    }

    pub fn new(sprite: Sprite,t:Arc<BlockType>) -> Slot{
        let pos = sprite.transform.pos.clone();
        return Slot { sprite, block: t,pos,size: vec3(22.0,22.0,0.0) }
    }

    pub fn set_block(&mut self,block:Arc<BlockType>) {
        self.block = block;
    }
}