use rust_game_library_wgpu::pipeline::pipeline::{RenderPipelineGroup, RenderPipelineGroupBuilder};
use wgpu::{SwapChainDescriptor, Device, PrimitiveTopology};
use rust_game_library_wgpu::objects::vertex_buffer::{VertexBuffer, OnlyCoordsVertexBuffer};
use rust_game_library_wgpu::objects::camera::Camera;

pub struct LineMeshRendererPipeline {
    pub group: RenderPipelineGroup
}

impl LineMeshRendererPipeline {
    pub fn new(vertex_shader_path : String, fragment_shader_path: String,sc_desc:&SwapChainDescriptor,device:&Device) -> LineMeshRendererPipeline {
        let mut group2_builder = RenderPipelineGroupBuilder::empty();
        group2_builder.set_shaders(&device,vertex_shader_path,fragment_shader_path,"vertex_line_mesh".to_string(),"fragment_line_mesh".to_string());

        group2_builder.bind_groups_layouts.push(Camera::bind_group_layout(device));
        group2_builder.bind_groups_layouts.push(Camera::transform_bind_group(device));

        let group2 = group2_builder.build(sc_desc,device,OnlyCoordsVertexBuffer::desc(),PrimitiveTopology::LineList,true);

        return LineMeshRendererPipeline { group: group2 }
    }
}