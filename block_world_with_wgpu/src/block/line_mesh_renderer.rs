use crate::block::line_mesh_renderer_pipeline::LineMeshRendererPipeline;
use wgpu::{SwapChainDescriptor, Device, BufferAddress, Queue, RenderPass};
use crate::block::line_mesh::LineMesh;
use rust_game_library_wgpu::objects::camera::Camera;
use nalgebra_glm::TVec3;

pub struct LineMeshRenderer {
    pub line_mesh_shader: LineMeshRendererPipeline
}

impl LineMeshRenderer {
    pub fn new(working_dir:String,device:&Device,sc:&SwapChainDescriptor) -> LineMeshRenderer {
        let group = LineMeshRendererPipeline::new(format!("{}\\{}",&working_dir.to_string(),"assets\\shader\\line_mesh\\vertex.shader"),format!("{}\\{}",&working_dir.to_string(),"assets\\shader\\line_mesh\\fragment.shader"),sc,device);
        return LineMeshRenderer { line_mesh_shader: group}
    }

    pub fn render_line_mesh_instant(&self, line_mesh:&mut LineMesh, offset:&mut BufferAddress, camera:&Camera, queue:&Queue, position: &TVec3<f32>) {
        queue.write_buffer(&camera.buffers[2],*offset,&*rust_game_library_wgpu::objects::matrix_helper::get_bytes(&nalgebra_glm::translation(position)));
        line_mesh.camera_offset = *offset as u32;
        *offset += 256;
    }

    pub fn finish_instant<'a>(&'a self, line_mesh:&'a LineMesh, render_pass:&mut RenderPass<'a>, camera:&'a Camera) {
        render_pass.set_pipeline(&self.line_mesh_shader.group.pipeline);
        render_pass.set_bind_group(0,&camera.bind_group,&[]);
        render_pass.set_bind_group(1,&camera.transform_bind_group,&[line_mesh.camera_offset as u32]);
        line_mesh.mesh.render(render_pass);
    }
}