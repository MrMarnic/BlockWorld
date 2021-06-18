use crate::world::chunk::chunk_render_pipeline::ChunkRenderPipelineGroup;
use wgpu::{Device, SwapChainDescriptor, BufferAddress, Queue, RenderPass};
use rust_game_library_wgpu::engine::sprite_renderer::SpriteRenderer;
use crate::world::chunk::chunk::Chunk;
use rust_game_library_wgpu::objects::camera::Camera;
use std::rc::Rc;
use rust_game_library_wgpu::objects::texture_map::TextureMap;
use crate::world::chunk::chunk_mesh::ChunkMesh;
use crate::world::chunk::draw_chunk::DrawChunk;

pub struct ChunkRenderer {
    pub shader: ChunkRenderPipelineGroup
}

impl ChunkRenderer {
    pub fn new(working_dir:String,device:&Device,sc:&SwapChainDescriptor) -> ChunkRenderer {
        unsafe {
            let shader = ChunkRenderPipelineGroup::new(format!("{}\\{}",&working_dir.to_string(),"assets\\shader\\chunk\\vertex.shader"),format!("{}\\{}",&working_dir.to_string(),"assets\\shader\\chunk\\fragment.shader"),sc,device);

            return ChunkRenderer { shader }
        }
    }

    pub fn begin_finish<'a>(&'a self, render_pass:&mut RenderPass<'a>, camera:&'a Camera,texture_map:&'a Rc<TextureMap>) {
        render_pass.set_pipeline(&self.shader.group.pipeline);
        render_pass.set_bind_group(0,&texture_map.texture.bind_group,&[]);
        render_pass.set_bind_group(1,&camera.bind_group,&[]);
        render_pass.set_bind_group(3,&self.shader.light_pos_group,&[]);
    }

    pub fn finish_instant<'a>(&'a self, chunk:&'a DrawChunk, render_pass:&mut RenderPass<'a>, camera:&'a Camera) {
        if chunk.mesh.is_some() {
            render_pass.set_bind_group(2,&camera.transform_bind_group,&[chunk.camera_offset as u32]);
            chunk.mesh.as_ref().unwrap().mesh.render(render_pass);
        }
    }
}